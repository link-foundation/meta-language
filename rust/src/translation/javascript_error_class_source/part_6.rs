// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_for_of;
use super::lower_java_script_call_source_iterator_loop;
use super::lower_java_script_call_source_iterator_remainder;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_read_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceBinding(identity) => {
            if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])))
            } else {
                {
                    let index = ({ let ml_reference_array54 = bindings.clone(); let ml_reference_projection54 = ml_reference_array54.borrow().len() as f64; ml_reference_projection54 } - 1f64);
                    {
                        let source_ml_source2_s4391 = self::source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18(bindings.clone(), state.clone(), identity, index);
                        match source_ml_source2_s4391.clone() {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done => {
                                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 104, 97, 115, 32, 110, 111, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114])))
                            }
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(source_ml_source1_temporary_result18) => {
                                let source_ml_source1_temporary_result18 = *source_ml_source1_temporary_result18;
                                source_ml_source1_temporary_result18.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceLet(source_identity, source_value, source_body) => {
            let source_value = *source_value;
            let source_body = *source_body;
            if (!(self::test_java_script_call_code_coordinate(source_identity, 65536f64)) || { let ml_values55 = { let ml_reference_value56 = bindings.borrow().clone(); ml_reference_value56 }; let ml_callback55 = { let source_identity = source_identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4393, _, _, _) => {
                    source_ml_source2_data_property_value4393
                }
            }) == source_identity) }) }; ml_values55.iter().cloned().enumerate().any(|(ml_index55, ml_value55)| ml_callback55(ml_value55)) }) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
            } else {
                {
                    let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    self::lower_java_script_call_source_expression(source_body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value4582 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                            let source_ml_source2_constructor_field4579 = match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4578) => {
                                    source_ml_source2_data_property_value4578
                                }
                            };
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity, source_ml_source2_constructor_field4579, false, None::<(Option<bool>,)>)
                        }]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value57 = bindings.borrow().clone(); ml_reference_value57 }), { let ml_reference_value58 = source_ml_source2_concat_value4582.borrow().clone(); ml_reference_value58 })
                    })), match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4583, _) => {
                            let source_ml_source2_data_property_value4583 = *source_ml_source2_data_property_value4583;
                            source_ml_source2_data_property_value4583.clone()
                        }
                    }, (depth + 1f64))
                }
            }
        }
        self::JavaScriptCallSource::SourceLocalLet(source_identity_2, source_value_2, source_body_2, source_ml_source2_f4328) => {
            let source_value_2 = *source_value_2;
            let source_body_2 = *source_body_2;
            if (!(self::test_java_script_call_code_coordinate(source_identity_2, 65536f64)) || { let ml_values59 = { let ml_reference_value60 = bindings.borrow().clone(); ml_reference_value60 }; let ml_callback59 = { let source_identity_2 = source_identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4585, _, _, _) => {
                    source_ml_source2_data_property_value4585
                }
            }) == source_identity_2) }) }; ml_values59.iter().cloned().enumerate().any(|(ml_index59, ml_value59)| ml_callback59(ml_value59)) }) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
            } else {
                {
                    let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let local = self::allocate_java_script_call_source_register(match value_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4769, _) => {
                                let source_ml_source2_data_property_value4769 = *source_ml_source2_data_property_value4769;
                                source_ml_source2_data_property_value4769.clone()
                            }
                        });
                        {
                            let copied = self::append_java_script_call_source_instruction(match local.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4770, _) => {
                                    let source_ml_source2_data_property_value4770 = *source_ml_source2_data_property_value4770;
                                    source_ml_source2_data_property_value4770.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match local.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4771) => {
                                    source_ml_source2_data_property_value4771
                                }
                            }, match value_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4772) => {
                                    source_ml_source2_data_property_value4772
                                }
                            }));
                            self::lower_java_script_call_source_expression(source_body_2.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value4778 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                    let source_ml_source2_constructor_field4775 = match local.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4774) => {
                                            source_ml_source2_data_property_value4774
                                        }
                                    };
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity_2, source_ml_source2_constructor_field4775, source_ml_source2_f4328, None::<(Option<bool>,)>)
                                }]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value61 = bindings.borrow().clone(); ml_reference_value61 }), { let ml_reference_value62 = source_ml_source2_concat_value4778.borrow().clone(); ml_reference_value62 })
                            })), copied.clone(), (depth + 1f64))
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceArgumentRest(from, source_prototype, realm) => {
            let source_prototype = *source_prototype;
            if ((!(self::test_java_script_call_code_coordinate(from, 1048576f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 102, 111, 114, 109, 97, 108, 32, 115, 116, 97, 114, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let result = self::allocate_java_script_call_source_register(match prototype.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4779, _) => {
                                let source_ml_source2_data_property_value4779 = *source_ml_source2_data_property_value4779;
                                source_ml_source2_data_property_value4779.clone()
                            }
                        });
                        {
                            let source_ml_source2_nominal_field4784 = match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4785, _) => {
                                    let source_ml_source2_data_property_value4785 = *source_ml_source2_data_property_value4785;
                                    source_ml_source2_data_property_value4785.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field4786 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4787) => {
                                        source_ml_source2_data_property_value4787
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override4788 = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4780, _) => {
                                            let source_ml_source2_data_property_value4780 = *source_ml_source2_data_property_value4780;
                                            source_ml_source2_data_property_value4780.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::ArgumentRest(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4781) => {
                                            source_ml_source2_data_property_value4781
                                        }
                                    }, match prototype.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4782) => {
                                            source_ml_source2_data_property_value4782
                                        }
                                    }, from, realm.clone()));
                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4788.clone()), source_ml_source2_nominal_field4786)
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceParameterRest(identity_2, from_2, source_prototype_2, realm_2, body) => {
            let source_prototype_2 = *source_prototype_2;
            let body = *body;
            {
                let binding = { let ml_values63 = { let ml_reference_value64 = bindings.borrow().clone(); ml_reference_value64 }; let ml_callback63 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4790, _, _, _) => {
                        source_ml_source2_data_property_value4790
                    }
                }) == identity_2) }) }; ml_values63.iter().cloned().enumerate().find_map(|(ml_index63, ml_value63)| if ml_callback63(ml_value63.clone()) { Some(ml_value63) } else { None }) };
                if (((((({
                    let source_ml_source2_comparison_operand4792 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4791, _, _, _) => {
                            ((source_ml_source2_data_property_value4791.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4792.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4795, _, _) => {
                        source_ml_source2_data_property_value4795
                    }
                }) != ((from_2 + 1f64)))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4798, _, _) => {
                        source_ml_source2_data_property_value4798
                    }
                }) > (match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4799, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value4799
                    }
                }))) || !(self::test_java_script_call_code_coordinate(from_2, 1048577f64))) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![114, 101, 115, 116, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
                } else {
                    {
                        let prototype_2 = self::lower_java_script_call_source_expression(source_prototype_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let copied_2 = self::append_java_script_call_source_instruction(match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4800, _) => {
                                    let source_ml_source2_data_property_value4800 = *source_ml_source2_data_property_value4800;
                                    source_ml_source2_data_property_value4800.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArgumentRest(match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4801, _, _) => {
                                    source_ml_source2_data_property_value4801
                                }
                            }, match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4802) => {
                                    source_ml_source2_data_property_value4802
                                }
                            }, from_2, realm_2.clone()));
                            self::lower_java_script_call_source_expression(body.clone(), bindings.clone(), copied_2.clone(), (depth + 1f64))
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceParameterDefault(identity_3, source_value_3, body_2) => {
            let source_value_3 = *source_value_3;
            let body_2 = *body_2;
            if {
                let source_ml_source2_comparison_operand4804 = match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4803, _, _, _) => {
                        ((source_ml_source2_data_property_value4803.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                !(match (source_ml_source2_comparison_operand4804.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            } {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 112, 97, 115, 115])))
            } else {
                {
                    let binding_2 = { let ml_values65 = { let ml_reference_value66 = bindings.borrow().clone(); ml_reference_value66 }; let ml_callback65 = { let identity_3 = identity_3; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4807, _, _, _) => {
                            source_ml_source2_data_property_value4807
                        }
                    }) == identity_3) }) }; ml_values65.iter().cloned().enumerate().find_map(|(ml_index65, ml_value65)| if ml_callback65(ml_value65.clone()) { Some(ml_value65) } else { None }) };
                    if ((match (binding_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding_2.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4809, _, _) => {
                            source_ml_source2_data_property_value4809
                        }
                    }) < (1f64))) || ((match binding_2.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4812, _, _) => {
                            source_ml_source2_data_property_value4812
                        }
                    }) > (match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4813, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value4813
                        }
                    }))) {
                        self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 100, 101, 102, 97, 117, 108, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])))
                    } else {
                        {
                            let condition = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourceBinding(identity_3)), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined)))), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let branch = { let ml_reference_array67 = match match condition.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4814, _) => {
                                        let source_ml_source2_data_property_value4814 = *source_ml_source2_data_property_value4814;
                                        source_ml_source2_data_property_value4814.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4815, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value4815.clone()
                                    }
                                }; let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 };
                                {
                                    let start = self::append_java_script_call_source_instruction(match condition.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4816, _) => {
                                            let source_ml_source2_data_property_value4816 = *source_ml_source2_data_property_value4816;
                                            source_ml_source2_data_property_value4816.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4817) => {
                                            source_ml_source2_data_property_value4817
                                        }
                                    }, 0f64, 0f64));
                                    {
                                        let value_3 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                        {
                                            let copied_3 = self::append_java_script_call_source_instruction(match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4818, _) => {
                                                    let source_ml_source2_data_property_value4818 = *source_ml_source2_data_property_value4818;
                                                    source_ml_source2_data_property_value4818.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match binding_2.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4819, _, _) => {
                                                    source_ml_source2_data_property_value4819
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4820) => {
                                                    source_ml_source2_data_property_value4820
                                                }
                                            }));
                                            {
                                                let end = { let ml_reference_array68 = match copied_3.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4821, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value4821.clone()
                                                    }
                                                }; let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 };
                                                {
                                                    let updated = {
                                                        let source_ml_source2_nominal_spread4824 = copied_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4825 = match source_ml_source2_nominal_spread4824.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4826, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value4826.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field4827 = match source_ml_source2_nominal_spread4824.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4828, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value4828
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4829 = match source_ml_source2_nominal_spread4824.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4830, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value4830
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field4831 = match source_ml_source2_nominal_spread4824.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4832, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value4832
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field4833 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4834, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value4834
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field4835 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4836, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value4836.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field4837 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4838, _, _, _) => {
                                                                                            source_ml_source2_data_property_value4838.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4839 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4840, _, _) => {
                                                                                                source_ml_source2_data_property_value4840.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4841 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4842, _) => {
                                                                                                    source_ml_source2_data_property_value4842.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4843 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4844) => {
                                                                                                        source_ml_source2_data_property_value4844.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override4845 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values69 = { let ml_reference_value70 = (match copied_3.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4822, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value4822.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value70 }; let ml_callback69 = { let branch = branch; let condition = condition.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index_2: f64| -> self::JavaScriptCallInstruction { if (index_2 == branch) {
                                                                                                        self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4823) => {
                                                                                                                source_ml_source2_data_property_value4823
                                                                                                            }
                                                                                                        }, (branch + 1f64), end)
                                                                                                    } else {
                                                                                                        instruction.clone()
                                                                                                    } }) }; ml_values69.iter().cloned().enumerate().map(|(ml_index69, ml_value69)| ml_callback69(ml_value69, (ml_index69 as f64))).collect::<Vec<_>>() }));
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override4845.clone(), source_ml_source2_nominal_field4827, source_ml_source2_nominal_field4829, source_ml_source2_nominal_field4831, source_ml_source2_nominal_field4833, source_ml_source2_nominal_field4835.clone(), source_ml_source2_nominal_field4837.clone(), source_ml_source2_nominal_field4839.clone(), source_ml_source2_nominal_field4841.clone(), source_ml_source2_nominal_field4843.clone())
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
                                                    };
                                                    {
                                                        let continuation = if match (match copied_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4846, _, _) => {
                                                                ((source_ml_source2_data_property_value4846.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            updated.clone()
                                                        } else {
                                                            {
                                                                let source_ml_source2_nominal_field4850 = match updated.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4851, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value4851.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4852 = match updated.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4853, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value4853
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field4854 = match updated.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4855, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value4855
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field4856 = match updated.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4857, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value4857
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field4858 = match updated.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4859, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value4859
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field4860 = match updated.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4861, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value4861.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4862 = match updated.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4863, _, _, _) => {
                                                                                                source_ml_source2_data_property_value4863.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4864 = match updated.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4865, _, _) => {
                                                                                                    source_ml_source2_data_property_value4865.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4866 = match updated.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4867, _) => {
                                                                                                        source_ml_source2_data_property_value4867.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4868 = match updated.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4869) => {
                                                                                                            source_ml_source2_data_property_value4869.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override4870 = (match match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4847, _) => {
                                                                                                                let source_ml_source2_data_property_value4847 = *source_ml_source2_data_property_value4847;
                                                                                                                source_ml_source2_data_property_value4847.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4848, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value4848.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field4850.clone(), source_ml_source2_nominal_field4852, source_ml_source2_nominal_field4854, source_ml_source2_nominal_field4856, source_ml_source2_nominal_field4858, source_ml_source2_nominal_field4860.clone(), source_ml_source2_nominal_field4862.clone(), Some((Some(source_ml_source2_nominal_override4870.clone()),)), source_ml_source2_nominal_field4866.clone(), source_ml_source2_nominal_field4868.clone())
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
                                                        };
                                                        self::lower_java_script_call_source_expression(body_2.clone(), bindings.clone(), continuation.clone(), (depth + 1f64))
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
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 114, 101, 97, 100, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorStep(slot, realm, read_value) => {
            if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 115, 116, 101, 112, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111, 110, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 99, 104, 111, 105, 99, 101])))
            } else {
                {
                    let value = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5050, _) => {
                                let source_ml_source2_data_property_value5050 = *source_ml_source2_data_property_value5050;
                                source_ml_source2_data_property_value5050.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5052 = match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5051) => {
                                    source_ml_source2_data_property_value5051
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5071 = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5053, _) => {
                                        let source_ml_source2_data_property_value5053 = *source_ml_source2_data_property_value5053;
                                        source_ml_source2_data_property_value5053.clone()
                                    }
                                }, {
                                    let source_ml_source1_temporary_constructor_field19 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5054) => {
                                            source_ml_source2_data_property_value5054
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_constructor_field20 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5055) => {
                                                source_ml_source2_data_property_value5055
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_constructor_field21 = 0f64;
                                            {
                                                let source_ml_source1_temporary_constructor_field22 = 0f64;
                                                if read_value {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                } else {
                                                    {
                                                        let source_ml_source2_constructor_field5070 = Some(false);
                                                        self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), Some((source_ml_source2_constructor_field5070.clone(),)))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5071.clone()), source_ml_source2_constructor_field5052)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::lower_java_script_call_source_iterator_remainder(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_iterator_loop(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorClose(slot_2, realm_2) => {
            if ((!(self::test_java_script_call_code_coordinate(slot_2, 65536f64)) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 111, 115, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let value_2 = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5072, _) => {
                                let source_ml_source2_data_property_value5072 = *source_ml_source2_data_property_value5072;
                                source_ml_source2_data_property_value5072.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5074 = match value_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5073) => {
                                    source_ml_source2_data_property_value5073
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5085 = self::append_java_script_call_source_instruction(match done_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5075, _) => {
                                        let source_ml_source2_data_property_value5075 = *source_ml_source2_data_property_value5075;
                                        source_ml_source2_data_property_value5075.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field5077 = match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5076) => {
                                            source_ml_source2_data_property_value5076
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field5079 = match done_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5078) => {
                                                source_ml_source2_data_property_value5078
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field5080 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field5081 = 0f64;
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field5077, source_ml_source2_constructor_field5079, source_ml_source2_constructor_field5080, source_ml_source2_constructor_field5081, slot_2, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), realm_2.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5085.clone()), source_ml_source2_constructor_field5074)
                            }
                        }
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
