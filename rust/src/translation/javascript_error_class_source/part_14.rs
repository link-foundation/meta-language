// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_iterator_scope;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call_source_control_scope;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_find_java_script_call_source_instruction_position_loop65;
use super::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63;
use super::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_loop(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceWhile(source_condition, source_body, after, source_target, source_update, post_test, iteration) => {
            let source_condition = *source_condition;
            let source_body = *source_body;
            let after = *after;
            {
                let target = ((source_target.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                {
                    let iteration_source = ((iteration.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                    {
                        let update_source = ((source_update.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let scoped = if {
                                let source_ml_source2_comparison_operand13521 = None::<()>;
                                match (target.clone(), source_ml_source2_comparison_operand13521.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                            } {
                                builder.clone()
                            } else {
                                self::push_java_script_call_source_control_scope(builder.clone(), target.clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112])))
                            };
                            if {
                                let source_ml_source2_comparison_operand13522 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                match (source_ml_source2_comparison_operand13522.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            } {
                                if (!(match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::refuse_java_script_call_source(scoped.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 112, 111, 115, 116, 45, 116, 101, 115, 116, 32, 108, 111, 111, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 119, 105, 116, 104, 111, 117, 116, 32, 97, 110, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 117, 112, 100, 97, 116, 101, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 104, 101, 97, 100, 32, 99, 108, 111, 110, 101])))
                                } else {
                                    {
                                        let start = { let ml_reference_array165 = match scoped.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13524, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value13524.clone()
                                            }
                                        }; let ml_reference_projection165 = ml_reference_array165.borrow().len() as f64; ml_reference_projection165 };
                                        {
                                            let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), scoped.clone(), (depth + 1f64));
                                            {
                                                let test = { let ml_reference_array166 = match match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13525, _) => {
                                                        let source_ml_source2_data_property_value13525 = *source_ml_source2_data_property_value13525;
                                                        source_ml_source2_data_property_value13525.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13526, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value13526.clone()
                                                    }
                                                }; let ml_reference_projection166 = ml_reference_array166.borrow().len() as f64; ml_reference_projection166 };
                                                {
                                                    let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), match body.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13527, _) => {
                                                            let source_ml_source2_data_property_value13527 = *source_ml_source2_data_property_value13527;
                                                            source_ml_source2_data_property_value13527.clone()
                                                        }
                                                    }, (depth + 1f64));
                                                    {
                                                        let end = ({ let ml_reference_array167 = match match condition.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13528, _) => {
                                                                let source_ml_source2_data_property_value13528 = *source_ml_source2_data_property_value13528;
                                                                source_ml_source2_data_property_value13528.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13529, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value13529.clone()
                                                            }
                                                        }; let ml_reference_projection167 = ml_reference_array167.borrow().len() as f64; ml_reference_projection167 } + 1f64);
                                                        {
                                                            let finished = self::append_java_script_call_source_instruction(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13530, _) => {
                                                                    let source_ml_source2_data_property_value13530 = *source_ml_source2_data_property_value13530;
                                                                    source_ml_source2_data_property_value13530.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13531) => {
                                                                    source_ml_source2_data_property_value13531
                                                                }
                                                            }, start, end));
                                                            {
                                                                let finished_3 = if {
                                                                    let source_ml_source2_comparison_operand13533 = None::<()>;
                                                                    !(match (target.clone(), source_ml_source2_comparison_operand13533.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                } {
                                                                    {
                                                                        let finished_2 = self::finish_java_script_call_source_control_scope(finished.clone(), builder.clone(), target.clone().expect("a proven present value"), end, test);
                                                                        finished_2.clone()
                                                                    }
                                                                } else {
                                                                    finished.clone()
                                                                };
                                                                self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished_3.clone(), (depth + 1f64))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let initial = if match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        scoped.clone()
                                    } else {
                                        match self::lower_java_script_call_source_expression(iteration_source.clone().expect("a proven present value"), bindings.clone(), scoped.clone(), (depth + 1f64)) {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13534, _) => {
                                                let source_ml_source2_data_property_value13534 = *source_ml_source2_data_property_value13534;
                                                source_ml_source2_data_property_value13534.clone()
                                            }
                                        }
                                    };
                                    {
                                        let start_2 = { let ml_reference_array168 = match initial.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13535, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value13535.clone()
                                            }
                                        }; let ml_reference_projection168 = ml_reference_array168.borrow().len() as f64; ml_reference_projection168 };
                                        {
                                            let condition_2 = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), initial.clone(), (depth + 1f64));
                                            {
                                                let branch = { let ml_reference_array169 = match match condition_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13536, _) => {
                                                        let source_ml_source2_data_property_value13536 = *source_ml_source2_data_property_value13536;
                                                        source_ml_source2_data_property_value13536.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13537, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value13537.clone()
                                                    }
                                                }; let ml_reference_projection169 = ml_reference_array169.borrow().len() as f64; ml_reference_projection169 };
                                                {
                                                    let test_2 = self::append_java_script_call_source_instruction(match condition_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13538, _) => {
                                                            let source_ml_source2_data_property_value13538 = *source_ml_source2_data_property_value13538;
                                                            source_ml_source2_data_property_value13538.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Branch(match condition_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13539) => {
                                                            source_ml_source2_data_property_value13539
                                                        }
                                                    }, (branch + 1f64), 0f64));
                                                    {
                                                        let body_2 = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), test_2.clone(), (depth + 1f64));
                                                        {
                                                            let update = { let ml_reference_array170 = match match body_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13540, _) => {
                                                                    let source_ml_source2_data_property_value13540 = *source_ml_source2_data_property_value13540;
                                                                    source_ml_source2_data_property_value13540.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13541, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value13541.clone()
                                                                }
                                                            }; let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 };
                                                            {
                                                                let cloned = if match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    match body_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13542, _) => {
                                                                            let source_ml_source2_data_property_value13542 = *source_ml_source2_data_property_value13542;
                                                                            source_ml_source2_data_property_value13542.clone()
                                                                        }
                                                                    }
                                                                } else {
                                                                    match self::lower_java_script_call_source_expression(iteration_source.clone().expect("a proven present value"), bindings.clone(), match body_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13543, _) => {
                                                                            let source_ml_source2_data_property_value13543 = *source_ml_source2_data_property_value13543;
                                                                            source_ml_source2_data_property_value13543.clone()
                                                                        }
                                                                    }, (depth + 1f64)) {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13544, _) => {
                                                                            let source_ml_source2_data_property_value13544 = *source_ml_source2_data_property_value13544;
                                                                            source_ml_source2_data_property_value13544.clone()
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let updated = if match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        cloned.clone()
                                                                    } else {
                                                                        match self::lower_java_script_call_source_expression(update_source.clone().expect("a proven present value"), bindings.clone(), cloned.clone(), (depth + 1f64)) {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13545, _) => {
                                                                                let source_ml_source2_data_property_value13545 = *source_ml_source2_data_property_value13545;
                                                                                source_ml_source2_data_property_value13545.clone()
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let loop_ = self::append_java_script_call_source_instruction(updated.clone(), self::JavaScriptCallInstruction::Jump(start_2));
                                                                        {
                                                                            let end_2 = { let ml_reference_array171 = match loop_.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13546, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value13546.clone()
                                                                                }
                                                                            }; let ml_reference_projection171 = ml_reference_array171.borrow().len() as f64; ml_reference_projection171 };
                                                                            {
                                                                                let finished_4 = {
                                                                                    let source_ml_source2_nominal_spread13549 = loop_.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13550 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13551, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value13551.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13552 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13553, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value13553
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13554 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13555, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value13555
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13556 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13557, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value13557
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field13558 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13559, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value13559
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field13560 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13561, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value13561.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field13562 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13563, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value13563.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field13564 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13565, _, _) => {
                                                                                                                            source_ml_source2_data_property_value13565.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field13566 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13567, _) => {
                                                                                                                                source_ml_source2_data_property_value13567.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field13568 = match source_ml_source2_nominal_spread13549.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13569) => {
                                                                                                                                    source_ml_source2_data_property_value13569.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override13570 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values172 = { let ml_reference_value173 = (match loop_.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13547, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value13547.clone()
                                                                                                                                    }
                                                                                                                                }).borrow().clone(); ml_reference_value173 }; let ml_callback172 = { let branch = branch; let condition_2 = condition_2.clone(); let end_2 = end_2; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                                    self::JavaScriptCallInstruction::Branch(match condition_2.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13548) => {
                                                                                                                                            source_ml_source2_data_property_value13548
                                                                                                                                        }
                                                                                                                                    }, (branch + 1f64), end_2)
                                                                                                                                } else {
                                                                                                                                    instruction.clone()
                                                                                                                                } }) }; ml_values172.iter().cloned().enumerate().map(|(ml_index172, ml_value172)| ml_callback172(ml_value172, (ml_index172 as f64))).collect::<Vec<_>>() }));
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override13570.clone(), source_ml_source2_nominal_field13552, source_ml_source2_nominal_field13554, source_ml_source2_nominal_field13556, source_ml_source2_nominal_field13558, source_ml_source2_nominal_field13560.clone(), source_ml_source2_nominal_field13562.clone(), source_ml_source2_nominal_field13564.clone(), source_ml_source2_nominal_field13566.clone(), source_ml_source2_nominal_field13568.clone())
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
                                                                                    let finished_6 = if {
                                                                                        let source_ml_source2_comparison_operand13572 = None::<()>;
                                                                                        !(match (target.clone(), source_ml_source2_comparison_operand13572.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                    } {
                                                                                        {
                                                                                            let finished_5 = self::finish_java_script_call_source_control_scope(finished_4.clone(), builder.clone(), target.clone().expect("a proven present value"), end_2, if (match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                start_2
                                                                                            } else {
                                                                                                update
                                                                                            });
                                                                                            finished_5.clone()
                                                                                        }
                                                                                    } else {
                                                                                        finished_4.clone()
                                                                                    };
                                                                                    self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished_6.clone(), (depth + 1f64))
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
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_switch(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let directory_error = self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 115, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
        match source.clone() {
            self::JavaScriptCallSource::SourceSwitch(target, value, clauses, after) => {
                let value = *value;
                let after = *after;
                if (({ let ml_reference_array174 = clauses.clone(); let ml_reference_projection174 = ml_reference_array174.borrow().len() as f64; ml_reference_projection174 }) > (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(directory_error))
                } else {
                    {
                        let input = self::lower_java_script_call_source_expression(value.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let saved = self::allocate_java_script_call_source_register(match input.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13753, _) => {
                                    let source_ml_source2_data_property_value13753 = *source_ml_source2_data_property_value13753;
                                    source_ml_source2_data_property_value13753.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13754, _) => {
                                        let source_ml_source2_data_property_value13754 = *source_ml_source2_data_property_value13754;
                                        source_ml_source2_data_property_value13754.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13755) => {
                                        source_ml_source2_data_property_value13755
                                    }
                                }, match input.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13756) => {
                                        source_ml_source2_data_property_value13756
                                    }
                                }));
                                {
                                    let current_2 = self::push_java_script_call_source_control_scope(current.clone(), target, &(self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104])));
                                    {
                                        let branches = std::rc::Rc::new(std::cell::RefCell::new(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                        {
                                            let fallback = (-1f64);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source2_s13757 = self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source.clone(), bindings.clone(), depth, saved.clone(), current_2.clone(), branches.clone(), fallback, index);
                                                    match source_ml_source2_s13757.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Done(current_3, fallback_2) => {
                                                            let current_3 = *current_3;
                                                            {
                                                                let dispatch = { let ml_reference_array175 = match current_3.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13758, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value13758.clone()
                                                                    }
                                                                }; let ml_reference_projection175 = ml_reference_array175.borrow().len() as f64; ml_reference_projection175 };
                                                                {
                                                                    let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                    {
                                                                        let bodies = std::rc::Rc::new(std::cell::RefCell::new(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                        {
                                                                            let source_ml_source1_temporary_values62 = clauses.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index62 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings.clone(), depth, current_4.clone(), bodies.clone(), source_ml_source1_temporary_values62.clone(), source_ml_source1_temporary_index62.clone());
                                                                                    {
                                                                                        let end = { let ml_reference_array176 = match current_5.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13759, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value13759.clone()
                                                                                            }
                                                                                        }; let ml_reference_projection176 = ml_reference_array176.borrow().len() as f64; ml_reference_projection176 };
                                                                                        {
                                                                                            let positions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values177 = { let ml_reference_value178 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value178 }; let ml_callback177 = std::rc::Rc::new(move |position: f64| -> bool { (position >= (0f64)) }); ml_values177.iter().cloned().enumerate().filter_map(|(ml_index177, ml_value177)| if ml_callback177(ml_value177.clone()) { Some(ml_value177) } else { None }).collect::<Vec<_>>() }));
                                                                                            {
                                                                                                let indices = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values179 = { let ml_reference_value180 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_values181 = { let ml_reference_value182 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value182 }; let ml_callback181 = std::rc::Rc::new(move |x_: f64, index_2: f64| -> f64 { index_2 }); ml_values181.iter().cloned().enumerate().map(|(ml_index181, ml_value181)| ml_callback181(ml_value181, (ml_index181 as f64))).collect::<Vec<_>>() }))).borrow().clone(); ml_reference_value180 }; let ml_callback179 = { let branches = branches.clone(); std::rc::Rc::new(move |index_2: f64| -> bool { ((({
                                                                                                    let source_ml_source2_array_read_value13762 = { let ml_cell_value = branches.borrow().clone(); ml_cell_value };
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index13763 = index_2;
                                                                                                        { let ml_reference_array183 = source_ml_source2_array_read_value13762.clone(); let ml_reference_position183 = source_ml_source2_array_read_index13763; let ml_reference_projection183 = self::ml_array::optional_at(&*ml_reference_array183.borrow(), self::ml_array::number_index(ml_reference_position183)); ml_reference_projection183 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) }) }; ml_values179.iter().cloned().enumerate().filter_map(|(ml_index179, ml_value179)| if ml_callback179(ml_value179.clone()) { Some(ml_value179) } else { None }).collect::<Vec<_>>() }));
                                                                                                {
                                                                                                    let fallback_target = if (fallback_2 < (0f64)) {
                                                                                                        end
                                                                                                    } else {
                                                                                                        ({
                                                                                                            let source_ml_source2_array_read_value13764 = { let ml_cell_value = bodies.borrow().clone(); ml_cell_value };
                                                                                                            { let ml_reference_array184 = source_ml_source2_array_read_value13764.clone(); let ml_reference_position184 = fallback_2; let ml_reference_projection184 = self::ml_array::optional_at(&*ml_reference_array184.borrow(), self::ml_array::number_index(ml_reference_position184)); ml_reference_projection184 }
                                                                                                        }).clone().unwrap_or_else(|| end)
                                                                                                    };
                                                                                                    {
                                                                                                        let current_6 = {
                                                                                                            let source_ml_source2_nominal_spread14018 = current_5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14019 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14020, _, _, _, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14020.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14021 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14022, _, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value14022
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14023 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14024, _, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value14024
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14025 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14026, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14026
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14027 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14028, _, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value14028
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14029 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14030, _, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value14030.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14031 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14032, _, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value14032.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14033 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14034, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value14034.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14035 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14036, _) => {
                                                                                                                                                        source_ml_source2_data_property_value14036.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field14037 = match source_ml_source2_nominal_spread14018.clone() {
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14038) => {
                                                                                                                                                            source_ml_source2_data_property_value14038.clone()
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override14039 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values185 = { let ml_reference_value186 = (match current_5.clone() {
                                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13766, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value13766.clone()
                                                                                                                                                            }
                                                                                                                                                        }).borrow().clone(); ml_reference_value186 }; let ml_callback185 = { let dispatch = dispatch; let fallback_target = fallback_target; let positions = positions.clone(); let indices = indices.clone(); let bodies = bodies.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index_2: f64| -> self::JavaScriptCallInstruction { if (index_2 == dispatch) {
                                                                                                                                                            self::JavaScriptCallInstruction::Jump(fallback_target)
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let selected = self::find_java_script_call_source_instruction_position(positions.clone(), index_2);
                                                                                                                                                                if (selected < (0f64)) {
                                                                                                                                                                    instruction.clone()
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let clause = {
                                                                                                                                                                            let source_ml_source2_array_read_value13767 = indices.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_array_read_index13768 = selected;
                                                                                                                                                                                { let ml_reference_array187 = source_ml_source2_array_read_value13767.clone(); let ml_reference_position187 = source_ml_source2_array_read_index13768; let ml_reference_projection187 = self::ml_array::optional_at(&*ml_reference_array187.borrow(), self::ml_array::number_index(ml_reference_position187)); ml_reference_projection187 }
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        if {
                                                                                                                                                                            let source_ml_source2_comparison_operand13769 = clause.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_comparison_operand13770 = None::<()>;
                                                                                                                                                                                match (source_ml_source2_comparison_operand13769.clone(), source_ml_source2_comparison_operand13770.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                                                                                            }
                                                                                                                                                                        } {
                                                                                                                                                                            instruction.clone()
                                                                                                                                                                        } else {
                                                                                                                                                                            match instruction.clone() {
                                                                                                                                                                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionToString(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PushFinally(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FinishFinally => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Jump(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Branch(condition, _, no) => {
                                                                                                                                                                                    self::JavaScriptCallInstruction::Branch(condition, ({
                                                                                                                                                                                        let source_ml_source2_array_read_value14016 = { let ml_cell_value = bodies.borrow().clone(); ml_cell_value };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_array_read_index14017 = clause.clone().expect("a proven present value");
                                                                                                                                                                                            { let ml_reference_array188 = source_ml_source2_array_read_value14016.clone(); let ml_reference_position188 = source_ml_source2_array_read_index14017; let ml_reference_projection188 = self::ml_array::optional_at(&*ml_reference_array188.borrow(), self::ml_array::number_index(ml_reference_position188)); ml_reference_projection188 }
                                                                                                                                                                                        }
                                                                                                                                                                                    }).clone().unwrap_or_else(|| end), no)
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Return(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Throw(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PopHandler => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } }) }; ml_values185.iter().cloned().enumerate().map(|(ml_index185, ml_value185)| ml_callback185(ml_value185, (ml_index185 as f64))).collect::<Vec<_>>() }));
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14039.clone(), source_ml_source2_nominal_field14021, source_ml_source2_nominal_field14023, source_ml_source2_nominal_field14025, source_ml_source2_nominal_field14027, source_ml_source2_nominal_field14029.clone(), source_ml_source2_nominal_field14031.clone(), source_ml_source2_nominal_field14033.clone(), source_ml_source2_nominal_field14035.clone(), source_ml_source2_nominal_field14037.clone())
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
                                                                                                            let current_7 = self::finish_java_script_call_source_control_scope(current_6.clone(), builder.clone(), target, end, (-1f64));
                                                                                                            self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), current_7.clone(), (depth + 1f64))
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
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(source_ml_source1_temporary_result64) => {
                                                            let source_ml_source1_temporary_result64 = *source_ml_source1_temporary_result64;
                                                            source_ml_source1_temporary_result64.clone()
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
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_source_instruction_position(positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array189 = positions.clone(); let ml_reference_projection189 = ml_reference_array189.borrow().len() as f64; ml_reference_projection189 };
            {
                let source_ml_source2_s14040 = self::source_ml_source1_temporary_find_java_script_call_source_instruction_position_loop65(positions.clone(), position, low, high);
                match source_ml_source2_s14040.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Return(source_ml_source1_temporary_result65) => {
                        source_ml_source1_temporary_result65
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_for_of(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceIteratorBinding(slot, input, iterator_key, body, realm) => {
            let input = *input;
            let iterator_key = *iterator_key;
            let body = *body;
            self::lower_java_script_call_source_iterator_scope(slot, realm.clone(), input.clone(), iterator_key.clone(), body.clone(), bindings.clone(), builder.clone(), depth)
        }
        self::JavaScriptCallSource::SourceForOf(slot_2, target, binding, mutable, input_2, iterator_key_2, source_body, realm_2, source_scope_bindings) => {
            let input_2 = *input_2;
            let iterator_key_2 = *iterator_key_2;
            let source_body = *source_body;
            {
                let scope_bindings = ((source_scope_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone();
                {
                    let body_2 = if !(match (scope_bindings.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        {
                            let source_ml_source2_constructor_field14217 = Some(scope_bindings.clone().expect("a proven present value"));
                            self::JavaScriptCallSource::SourceIteratorLoop(slot_2, target, binding, mutable, Box::new(source_body.clone()), realm_2.clone(), Some((source_ml_source2_constructor_field14217.clone(),)))
                        }
                    } else {
                        self::JavaScriptCallSource::SourceIteratorLoop(slot_2, target, binding, mutable, Box::new(source_body.clone()), realm_2.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
                    };
                    self::lower_java_script_call_source_iterator_scope(slot_2, realm_2.clone(), input_2.clone(), iterator_key_2.clone(), body_2.clone(), bindings.clone(), builder.clone(), depth)
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
        }
    }
}
