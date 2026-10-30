// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result;
use super::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop112Result;
use super::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop114Result;
use super::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop111Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_source_exception_operation;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop108;
use super::source_ml_source1_temporary_construct_java_script_literal_error_source_loop109;
use super::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop112;
use super::source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop114;
use super::source_ml_source1_temporary_test_java_script_natural_source_realm_loop111;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, shared_constructor_field15, _) => {
            match source.clone() {
                self::JavaScriptCallSource::SourceConstant(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceReceiver => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceArgument(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceThrow(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
                    let source_body = *source_body;
                    let source_handler = *source_handler;
                    if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value694, _) => {
                            source_ml_source2_data_property_value694
                        }
                    }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                        self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
                    } else {
                        if (shared_constructor_field15 >= (1024f64)) {
                            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
                        } else {
                            {
                                let caught = self::allocate_java_script_call_source_register(builder.clone());
                                match caught.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field13, shared_constructor_field14) => {
                                        let shared_constructor_field13 = *shared_constructor_field13;
                                        {
                                            let result = self::allocate_java_script_call_source_register(shared_constructor_field13.clone());
                                            match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field11, shared_constructor_field12) => {
                                                    let shared_constructor_field11 = *shared_constructor_field11;
                                                    {
                                                        let position = ((match &shared_constructor_field11 { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value699, _, _, _, _, _) => source_ml_source2_data_property_value699.as_slice() }).len() as f64);
                                                        {
                                                            let protected_builder = self::append_java_script_call_source_instruction({
                                                                let source_ml_source2_nominal_spread702 = shared_constructor_field11.clone();
                                                                match source_ml_source2_nominal_spread702.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                        {
                                                                            let source_ml_source2_nominal_field703 = shared_constructor_field1;
                                                                            {
                                                                                let source_ml_source2_nominal_field705 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field707 = shared_constructor_field3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field709 = shared_constructor_field4;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field711 = shared_constructor_field5;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field713 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override715 = (shared_constructor_field15 + 1f64);
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field703, source_ml_source2_nominal_field705, source_ml_source2_nominal_field707, source_ml_source2_nominal_field709, source_ml_source2_nominal_override715, source_ml_source2_nominal_field713.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }, self::JavaScriptCallInstruction::PushHandler(shared_constructor_field14, 0f64));
                                                            {
                                                                let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                                                match body.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field9 = *shared_constructor_field9;
                                                                        {
                                                                            let popped = self::append_java_script_call_source_instruction({
                                                                                let source_ml_source2_nominal_spread719 = shared_constructor_field9.clone();
                                                                                match source_ml_source2_nominal_spread719.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                                                                        {
                                                                                            let source_ml_source2_nominal_field720 = shared_constructor_field1_2;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field722 = shared_constructor_field2_2;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field724 = shared_constructor_field3_2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field726 = shared_constructor_field4_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field728 = shared_constructor_field5_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field730 = shared_constructor_field6_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override732 = shared_constructor_field15;
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field720, source_ml_source2_nominal_field722, source_ml_source2_nominal_field724, source_ml_source2_nominal_field726, source_ml_source2_nominal_override732, source_ml_source2_nominal_field730.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }, self::JavaScriptCallInstruction::PopHandler);
                                                                            {
                                                                                let copied = self::append_java_script_call_source_instruction(popped.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field10));
                                                                                {
                                                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value735, _, _, _, _, _) => source_ml_source2_data_property_value735.as_slice() }).len() as f64);
                                                                                    {
                                                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                                        {
                                                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value736, _, _, _, _, _) => source_ml_source2_data_property_value736.as_slice() }).len() as f64);
                                                                                            {
                                                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                                                    let source_ml_source2_concat_value739 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, shared_constructor_field14)];
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), bindings), source_ml_source2_concat_value739)
                                                                                                }, alternative.clone(), (depth + 1f64));
                                                                                                match handler.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field7, shared_constructor_field8) => {
                                                                                                        let shared_constructor_field7 = *shared_constructor_field7;
                                                                                                        {
                                                                                                            let finished = self::append_java_script_call_source_instruction(shared_constructor_field7.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field8));
                                                                                                            match finished.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                                                                                                    {
                                                                                                                        let end = (shared_constructor_field1_3.len() as f64);
                                                                                                                        if (shared_constructor_field6_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let instructions = { let ml_values3 = shared_constructor_field1_3.clone(); let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value747) => {
                                                                                                                                            source_ml_source2_data_property_value747
                                                                                                                                        }
                                                                                                                                    }, target)
                                                                                                                                } else {
                                                                                                                                    if (index == jump) {
                                                                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                    } else {
                                                                                                                                        entry.clone()
                                                                                                                                    }
                                                                                                                                } }) }; ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                                                                                    let source_ml_source2_nominal_field749 = shared_constructor_field1_3;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field751 = shared_constructor_field2_3;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field753 = shared_constructor_field3_3;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field755 = shared_constructor_field4_3;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field757 = shared_constructor_field5_3;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field759 = shared_constructor_field6_3.clone();
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(instructions, source_ml_source2_nominal_field751, source_ml_source2_nominal_field753, source_ml_source2_nominal_field755, source_ml_source2_nominal_field757, source_ml_source2_nominal_field759.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }), shared_constructor_field12)
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallSource::SourceBinding(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceLet(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceIf(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceSameValue(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceRecur(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceSet(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceGet(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceCall(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                match condition.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field13, shared_constructor_field14) => {
                        let shared_constructor_field13 = *shared_constructor_field13;
                        {
                            let result = self::allocate_java_script_call_source_register(shared_constructor_field13.clone());
                            match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field11, shared_constructor_field12) => {
                                    let shared_constructor_field11 = *shared_constructor_field11;
                                    {
                                        let branch = ((match &shared_constructor_field11 { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value793, _, _, _, _, _) => source_ml_source2_data_property_value793.as_slice() }).len() as f64);
                                        {
                                            let start = self::append_java_script_call_source_instruction(shared_constructor_field11.clone(), self::JavaScriptCallInstruction::Branch(shared_constructor_field14, 0f64, 0f64));
                                            {
                                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value796, _, _, _, _, _) => source_ml_source2_data_property_value796.as_slice() }).len() as f64);
                                                {
                                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                                    match yes.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field9, shared_constructor_field10) => {
                                                            let shared_constructor_field9 = *shared_constructor_field9;
                                                            {
                                                                let copied_yes = self::append_java_script_call_source_instruction(shared_constructor_field9.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field10));
                                                                {
                                                                    let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value800, _, _, _, _, _) => source_ml_source2_data_property_value800.as_slice() }).len() as f64);
                                                                    {
                                                                        let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                        {
                                                                            let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value801, _, _, _, _, _) => source_ml_source2_data_property_value801.as_slice() }).len() as f64);
                                                                            {
                                                                                let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                                                match no.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field7, shared_constructor_field8) => {
                                                                                        let shared_constructor_field7 = *shared_constructor_field7;
                                                                                        {
                                                                                            let copied_no = self::append_java_script_call_source_instruction(shared_constructor_field7.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field8));
                                                                                            match copied_no.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                                                    {
                                                                                                        let end = (shared_constructor_field1.len() as f64);
                                                                                                        if (shared_constructor_field6 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                                                        } else {
                                                                                                            {
                                                                                                                let instructions = { let ml_values4 = shared_constructor_field1.clone(); let ml_callback4 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                    self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value809) => {
                                                                                                                            source_ml_source2_data_property_value809
                                                                                                                        }
                                                                                                                    }, yes_position, no_position)
                                                                                                                } else {
                                                                                                                    if (index == jump) {
                                                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                                                    } else {
                                                                                                                        entry.clone()
                                                                                                                    }
                                                                                                                } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                                                                    let source_ml_source2_nominal_field811 = shared_constructor_field1;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field813 = shared_constructor_field2;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field815 = shared_constructor_field3;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field817 = shared_constructor_field4;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field819 = shared_constructor_field5;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field821 = shared_constructor_field6.clone();
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(instructions, source_ml_source2_nominal_field813, source_ml_source2_nominal_field815, source_ml_source2_nominal_field817, source_ml_source2_nominal_field819, source_ml_source2_nominal_field821.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }), shared_constructor_field12)
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
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceGet(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    if (((parameters.len() as f64)) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s825 = self::source_ml_source1_temporary_compile_java_script_call_source_loop108(parameters, index);
                match source_ml_source2_s825.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Done => {
                        {
                            let bindings = { let ml_values5 = parameters.to_vec(); let ml_callback5 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), 0f64, self::ml_string::Text::from_units(vec![])), 0f64);
                                match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field4, shared_constructor_field5) => {
                                        let shared_constructor_field4 = *shared_constructor_field4;
                                        {
                                            let builder = self::append_java_script_call_source_instruction(shared_constructor_field4.clone(), self::JavaScriptCallInstruction::Return(shared_constructor_field5));
                                            match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, _, _, _, shared_constructor_field3) => {
                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![]))) {
                                                        shared_constructor_field1
                                                    } else {
                                                        Vec::<self::JavaScriptCallInstruction>::new()
                                                    }, if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![]))) {
                                                        shared_constructor_field2
                                                    } else {
                                                        0f64
                                                    }, shared_constructor_field3.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Return(source_ml_source1_temporary_result108) => {
                        let source_ml_source1_temporary_result108 = *source_ml_source1_temporary_result108;
                        source_ml_source1_temporary_result108.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_literal_error_source(message: &self::ml_string::Text) -> self::JavaScriptCallSource {
    {
        let units = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let units_2 = self::source_ml_source1_temporary_construct_java_script_literal_error_source_loop109(message, units, index);
                self::JavaScriptCallSource::SourcePrimitive(self::read_java_script_source_exception_operation(&(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))), vec![self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(units_2))), self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(true)))])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_negative_source(identity: f64) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceNumberBinary(self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), vec![self::JavaScriptCallSource::SourceBinding(identity), self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))))])), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Number(0f64)))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_guard_source(identity: f64, message: &self::ml_string::Text, body: self::JavaScriptCallSource) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceIf(Box::new(self::construct_java_script_natural_negative_source(identity)), Box::new(self::construct_java_script_literal_error_source(message)), Box::new(body.clone()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_cast_source(identity: f64, message: &self::ml_string::Text, clamp: bool) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceIf(Box::new(self::construct_java_script_natural_negative_source(identity)), Box::new(if clamp {
        self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))))
    } else {
        self::construct_java_script_literal_error_source(message)
    }), Box::new(self::JavaScriptCallSource::SourceBinding(identity)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_difference_source(left: f64, right: f64) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceNumberBinary(self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), vec![self::JavaScriptCallSource::SourceBinding(left), self::JavaScriptCallSource::SourceBinding(right)])), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Number(0f64)))))), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))))), Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), vec![self::JavaScriptCallSource::SourceBinding(left), self::JavaScriptCallSource::SourceBinding(right)])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_predecessor_source(identity: f64) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), vec![self::JavaScriptCallSource::SourceBinding(identity), self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(1))))])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_natural_match_source(identity: f64, zero: self::JavaScriptCallSource, succ: self::JavaScriptCallSource) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourceBinding(identity)), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))))))), Box::new(zero.clone()), Box::new(succ.clone()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_natural_source_realm(realms: &[self::ml_string::Text]) -> bool {
    if (((realms.len() as f64)) == (0f64)) {
        false
    } else {
        {
            let first = (self::ml_array::optional_at(realms, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
            {
                let source_ml_source1_temporary_index110 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s835 = self::source_ml_source1_temporary_test_java_script_natural_source_realm_loop111(&(first), realms, source_ml_source1_temporary_index110.clone());
                    match source_ml_source2_s835.clone() {
                        self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop111Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop111Done => {
                            true
                        }
                        self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop111Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop111Return(source_ml_source1_temporary_result111) => {
                            source_ml_source1_temporary_result111
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_natural_guard_order(types: &[self::ml_string::Text], orders: &[f64], one_realm: bool) -> Vec<f64> {
    if (((((types.len() as f64)) != ((orders.len() as f64))) || (((types.len() as f64)) > (4096f64))) || ((((types.len() as f64)) > (0f64)) && !one_realm)) {
        vec![(-1f64)]
    } else {
        {
            let positions = Vec::<f64>::new();
            {
                let order = 0f64;
                {
                    let source_ml_source2_s836 = self::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop112(types, orders, positions, order);
                    match source_ml_source2_s836.clone() {
                        self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop112Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop112Done(positions_2) => {
                            positions_2
                        }
                        self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop112Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop112Return(source_ml_source1_temporary_result112) => {
                            source_ml_source1_temporary_result112
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_natural_cast_plan(from: &self::ml_string::Text, to: &self::ml_string::Text, flavor: &self::ml_string::Text, retained_message: bool, order: f64, one_realm: bool) -> f64 {
    if ((((*from) == (self::ml_string::Text::from_units(vec![110, 97, 116]))) && ((*to) == (self::ml_string::Text::from_units(vec![105, 110, 116])))) && ((*flavor) == (self::ml_string::Text::from_units(vec![101, 120, 97, 99, 116])))) {
        1f64
    } else {
        if (((*from) != (self::ml_string::Text::from_units(vec![105, 110, 116]))) || ((*to) != (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            0f64
        } else {
            if ((*flavor) == (self::ml_string::Text::from_units(vec![99, 108, 97, 109, 112]))) {
                2f64
            } else {
                if ((((((*flavor) == (self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100]))) && retained_message) && self::ml_math::is_safe_integer(order)) && (order >= (0f64))) && one_realm) {
                    3f64
                } else {
                    0f64
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_natural_pattern_binding(pattern: &self::ml_string::Text) -> f64 {
    if ((*pattern) == (self::ml_string::Text::from_units(vec![110, 97, 116, 83, 117, 99, 99]))) {
        2f64
    } else {
        if ((*pattern) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100]))) {
            1f64
        } else {
            if (((*pattern) == (self::ml_string::Text::from_units(vec![110, 97, 116, 90, 101, 114, 111]))) || ((*pattern) == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])))) {
                0f64
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_natural_pattern_branches(kind: &self::ml_string::Text, patterns: &[self::ml_string::Text]) -> Vec<f64> {
    if (((*kind) != (self::ml_string::Text::from_units(vec![110, 97, 116]))) || (((patterns.len() as f64)) > (3f64))) {
        Vec::<f64>::new()
    } else {
        {
            let zero = (-1f64);
            {
                let succ = (-1f64);
                {
                    let fallback = (-1f64);
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s837 = self::source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop114(patterns, zero, succ, fallback, index);
                            match source_ml_source2_s837.clone() {
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop114Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop114Done(zero_2, succ_2, fallback_2) => {
                                    {
                                        let zero_3 = if (zero_2 == ((-1f64))) {
                                            fallback_2
                                        } else {
                                            zero_2
                                        };
                                        {
                                            let succ_3 = if (succ_2 == ((-1f64))) {
                                                fallback_2
                                            } else {
                                                succ_2
                                            };
                                            if ((zero_3 < (0f64)) || (succ_3 < (0f64))) {
                                                Vec::<f64>::new()
                                            } else {
                                                vec![zero_3, succ_3]
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop114Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop114Return(source_ml_source1_temporary_result114) => {
                                    source_ml_source1_temporary_result114
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
pub fn compile_java_script_natural_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source(source.clone(), parameters)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand871 = {
                        let source_ml_source2_nullish_field_receiver865 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver865.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present866 = ((source_ml_source2_nullish_field_receiver865.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present866.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value867, _) => {
                                        source_ml_source2_data_property_value867
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand872 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceReceiver => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceArgument(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceThrow(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceTry(source_ml_source2_data_property_value868, _, _) => {
                                Some(source_ml_source2_data_property_value868)
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value869) => {
                                Some(source_ml_source2_data_property_value869)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value870, _, _) => {
                                Some(source_ml_source2_data_property_value870)
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
                            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceRecur(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceGet(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                                None::<f64>
                            }
                        };
                        match (source_ml_source2_comparison_operand871, source_ml_source2_comparison_operand872.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver873 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver873.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present874 = ((source_ml_source2_nullish_field_receiver873.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present874.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value875) => {
                                        source_ml_source2_data_property_value875
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values100: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index100: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    loop {
        return if (source_ml_source1_temporary_index100 < (self::ml::Big::from_u128(source_ml_source1_temporary_values100.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1, shared_constructor_field2) => {
                            let shared_constructor_field1 = *shared_constructor_field1;
                            {
                                let current_2 = shared_constructor_field1.clone();
                                {
                                    let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![shared_constructor_field2], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                    {
                                        let source_ml_source1_temporary_index100_2 = source_ml_source1_temporary_index100.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, arguments_, source_ml_source1_temporary_index100) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index100_2.clone());
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop106(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ((arguments_.len() as f64))) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), (self::ml_array::optional_at(arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop105(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values101: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index101: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    loop {
        return if (source_ml_source1_temporary_index101 < (self::ml::Big::from_u128(source_ml_source1_temporary_values101.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3, shared_constructor_field4) => {
                            let shared_constructor_field3 = *shared_constructor_field3;
                            {
                                let captured = self::allocate_java_script_call_source_register(shared_constructor_field3.clone());
                                match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1, shared_constructor_field2) => {
                                        let shared_constructor_field1 = *shared_constructor_field1;
                                        {
                                            let current_2 = self::append_java_script_call_source_instruction(shared_constructor_field1.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field2, shared_constructor_field4));
                                            {
                                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![shared_constructor_field2], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                                {
                                                    let source_ml_source1_temporary_index101_2 = source_ml_source1_temporary_index101.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (current, arguments_, source_ml_source1_temporary_index101) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index101_2.clone());
                                                        continue;
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(Box::new(current.clone()), arguments_)
        };
    }
}
