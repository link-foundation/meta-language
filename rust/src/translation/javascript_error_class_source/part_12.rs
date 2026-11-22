// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match condition.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12577, _) => {
                            let source_ml_source2_data_property_value12577 = *source_ml_source2_data_property_value12577;
                            source_ml_source2_data_property_value12577.clone()
                        }
                    });
                    {
                        let branch = { let ml_reference_array120 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12578, _) => {
                                let source_ml_source2_data_property_value12578 = *source_ml_source2_data_property_value12578;
                                source_ml_source2_data_property_value12578.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12579, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12579.clone()
                            }
                        }; let ml_reference_projection120 = ml_reference_array120.borrow().len() as f64; ml_reference_projection120 };
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12580, _) => {
                                    let source_ml_source2_data_property_value12580 = *source_ml_source2_data_property_value12580;
                                    source_ml_source2_data_property_value12580.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12581) => {
                                    source_ml_source2_data_property_value12581
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = { let ml_reference_array121 = match start.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12582, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12582.clone()
                                    }
                                }; let ml_reference_projection121 = ml_reference_array121.borrow().len() as f64; ml_reference_projection121 };
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12583, _) => {
                                                let source_ml_source2_data_property_value12583 = *source_ml_source2_data_property_value12583;
                                                source_ml_source2_data_property_value12583.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12584) => {
                                                source_ml_source2_data_property_value12584
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12585) => {
                                                source_ml_source2_data_property_value12585
                                            }
                                        }));
                                        {
                                            let jump = { let ml_reference_array122 = match copied_yes.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12586, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12586.clone()
                                                }
                                            }; let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 };
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = { let ml_reference_array123 = match alternative.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12587, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12587.clone()
                                                        }
                                                    }; let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 };
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings.clone(), if match (match alternative.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12588, _, _) => {
                                                                ((source_ml_source2_data_property_value12588.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            alternative.clone()
                                                        } else {
                                                            {
                                                                let source_ml_source2_nominal_field12592 = match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12593, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12593.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12594 = match alternative.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12595, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12595
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12596 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12597, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12597
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12598 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12599, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12599
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12600 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12601, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12601
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12602 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12603, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12603.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12604 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12605, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12605.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12606 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12607, _, _) => {
                                                                                                    source_ml_source2_data_property_value12607.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12608 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12609, _) => {
                                                                                                        source_ml_source2_data_property_value12609.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12610 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12611) => {
                                                                                                            source_ml_source2_data_property_value12611.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override12612 = (match match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12589, _) => {
                                                                                                                let source_ml_source2_data_property_value12589 = *source_ml_source2_data_property_value12589;
                                                                                                                source_ml_source2_data_property_value12589.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12590, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value12590.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12592.clone(), source_ml_source2_nominal_field12594, source_ml_source2_nominal_field12596, source_ml_source2_nominal_field12598, source_ml_source2_nominal_field12600, source_ml_source2_nominal_field12602.clone(), source_ml_source2_nominal_field12604.clone(), Some((Some(source_ml_source2_nominal_override12612.clone()),)), source_ml_source2_nominal_field12608.clone(), source_ml_source2_nominal_field12610.clone())
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
                                                        }, (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12613, _) => {
                                                                    let source_ml_source2_data_property_value12613 = *source_ml_source2_data_property_value12613;
                                                                    source_ml_source2_data_property_value12613.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12614) => {
                                                                    source_ml_source2_data_property_value12614
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12615) => {
                                                                    source_ml_source2_data_property_value12615
                                                                }
                                                            }));
                                                            {
                                                                let end = { let ml_reference_array124 = match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12616, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12616.clone()
                                                                    }
                                                                }; let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 };
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12618, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12618.clone()
                                                                    }
                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values125 = { let ml_reference_value126 = (match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12619, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12619.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value126 }; let ml_callback125 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12620) => {
                                                                                    source_ml_source2_data_property_value12620
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values125.iter().cloned().enumerate().map(|(ml_index125, ml_value125)| ml_callback125(ml_value125, (ml_index125 as f64))).collect::<Vec<_>>() }));
                                                                        {
                                                                            let updated = {
                                                                                let source_ml_source2_nominal_field12622 = match copied_no.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12623, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12623.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12624 = match copied_no.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12625, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12625
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12626 = match copied_no.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12627, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12627
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12628 = match copied_no.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12629, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12629
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12630 = match copied_no.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12631, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12631
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12632 = match copied_no.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12633, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12633.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12634 = match copied_no.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12635, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12635.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12636 = match copied_no.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12637, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12637.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12638 = match copied_no.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12639, _) => {
                                                                                                                        source_ml_source2_data_property_value12639.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12640 = match copied_no.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12641) => {
                                                                                                                            source_ml_source2_data_property_value12641.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12624, source_ml_source2_nominal_field12626, source_ml_source2_nominal_field12628, source_ml_source2_nominal_field12630, source_ml_source2_nominal_field12632.clone(), source_ml_source2_nominal_field12634.clone(), source_ml_source2_nominal_field12636.clone(), source_ml_source2_nominal_field12638.clone(), source_ml_source2_nominal_field12640.clone())
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
                                                                                let continuation = if (match (match match yes.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12643, _) => {
                                                                                        let source_ml_source2_data_property_value12643 = *source_ml_source2_data_property_value12643;
                                                                                        source_ml_source2_data_property_value12643.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12644, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12644.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match no.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12645, _) => {
                                                                                        let source_ml_source2_data_property_value12645 = *source_ml_source2_data_property_value12645;
                                                                                        source_ml_source2_data_property_value12645.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12646, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12646.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    updated.clone()
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_nominal_spread12653 = updated.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12654 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12655, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12655.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12656 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12657, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12657
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12658 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12659, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12659
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12660 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12661, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12661
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12662 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12663, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12663
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12664 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12665, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12665.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12666 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12667, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12667.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12668 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12669, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12669.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12670 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12671, _) => {
                                                                                                                                    source_ml_source2_data_property_value12671.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12672 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12673) => {
                                                                                                                                        source_ml_source2_data_property_value12673.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override12674 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values127 = { let ml_reference_value128 = ((match match yes.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12647, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12647 = *source_ml_source2_data_property_value12647;
                                                                                                                                            source_ml_source2_data_property_value12647.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12648, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12648.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value128 }; let ml_callback127 = { let no = no.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value129 = ((match match no.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12651, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12651 = *source_ml_source2_data_property_value12651;
                                                                                                                                            source_ml_source2_data_property_value12651.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12652, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12652.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value129 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values127.iter().cloned().enumerate().filter_map(|(ml_index127, ml_value127)| if ml_callback127(ml_value127.clone()) { Some(ml_value127) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12654.clone(), source_ml_source2_nominal_field12656, source_ml_source2_nominal_field12658, source_ml_source2_nominal_field12660, source_ml_source2_nominal_field12662, source_ml_source2_nominal_field12664.clone(), source_ml_source2_nominal_field12666.clone(), Some((Some(source_ml_source2_nominal_override12674.clone()),)), source_ml_source2_nominal_field12670.clone(), source_ml_source2_nominal_field12672.clone())
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
                                                                                };
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(continuation.clone()), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12675) => {
                                                                                        source_ml_source2_data_property_value12675
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
            }
        }
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 114, 97, 110, 99, 104, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_with_captures(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    {
        let first = self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), false);
        if ((match first.clone() {
            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value12677) => {
                source_ml_source2_data_property_value12677.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 112, 97, 115, 115]))) {
            self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), true)
        } else {
            first.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_mutable_call_source_with_captures(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_binding_reads(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, snapshot_mutable_bindings: bool) -> self::JavaScriptCallSourceCompilation {
    if ((({ let ml_reference_array130 = parameters.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 } + { let ml_reference_array131 = captures.clone(); let ml_reference_projection131 = ml_reference_array131.borrow().len() as f64; ml_reference_projection131 })) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s12678 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58(parameters.clone(), index);
                match source_ml_source2_s12678.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s12679 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59(parameters.clone(), captures.clone(), index_2);
                                match source_ml_source2_s12679.clone() {
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done => {
                                        {
                                            let bindings = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values132 = { let ml_reference_value133 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value134 = parameters.borrow().clone(); ml_reference_value134 }), { let ml_reference_value135 = captures.borrow().clone(); ml_reference_value135 })))).borrow().clone(); ml_reference_value133 }; let ml_callback132 = { let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index_3: f64| -> self::JavaScriptCallSourceBinding { {
                                                let source_ml_source2_constructor_field12682 = identity;
                                                {
                                                    let source_ml_source2_constructor_field12683 = (index_3 + 1f64);
                                                    {
                                                        let source_ml_source2_constructor_field12684 = (index_3 < ({ let ml_reference_array136 = parameters.clone(); let ml_reference_projection136 = ml_reference_array136.borrow().len() as f64; ml_reference_projection136 }));
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field12682, source_ml_source2_constructor_field12683, source_ml_source2_constructor_field12684, None::<(Option<bool>,)>)
                                                    }
                                                }
                                            } }) }; ml_values132.iter().cloned().enumerate().map(|(ml_index132, ml_value132)| ml_callback132(ml_value132, (ml_index132 as f64))).collect::<Vec<_>>() }));
                                            {
                                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings.clone(), {
                                                    let source_ml_source1_temporary_constructor_field52 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                                                    {
                                                        let source_ml_source1_temporary_constructor_field53 = (({ let ml_reference_array137 = parameters.clone(); let ml_reference_projection137 = ml_reference_array137.borrow().len() as f64; ml_reference_projection137 } + { let ml_reference_array138 = captures.clone(); let ml_reference_projection138 = ml_reference_array138.borrow().len() as f64; ml_reference_projection138 }) + 1f64);
                                                        {
                                                            let source_ml_source1_temporary_constructor_field54 = 0f64;
                                                            {
                                                                let source_ml_source1_temporary_constructor_field55 = { let ml_reference_array139 = parameters.clone(); let ml_reference_projection139 = ml_reference_array139.borrow().len() as f64; ml_reference_projection139 };
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field56 = 0f64;
                                                                    if snapshot_mutable_bindings {
                                                                        {
                                                                            let source_ml_source2_constructor_field12691 = Some(true);
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, self::ml_string::Text::from_units(vec![]), Some((source_ml_source2_constructor_field12691.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                        }
                                                                    } else {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, self::ml_string::Text::from_units(vec![]), None::<(Option<bool>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }, 0f64);
                                                {
                                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12698, _) => {
                                                            let source_ml_source2_data_property_value12698 = *source_ml_source2_data_property_value12698;
                                                            source_ml_source2_data_property_value12698.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12699) => {
                                                            source_ml_source2_data_property_value12699
                                                        }
                                                    }));
                                                    {
                                                        let builder_3 = if { let ml_value = { let ml_reference_array141 = (match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12700) => {
                                                                ((source_ml_source2_data_property_value12700.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                                            }
                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new()))); let ml_reference_projection141 = ml_reference_array141.borrow().len() as f64; ml_reference_projection141 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                            {
                                                                let builder_2 = {
                                                                    let source_ml_source2_nominal_field12702 = match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12703, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12703.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12704 = match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12705, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12705
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12706 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12707, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12707
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12708 = match builder.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12709, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12709
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12710 = match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12711, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12711
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12712 = match builder.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12713, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12713.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12714 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12715, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12715.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12716 = match builder.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12717, _, _) => {
                                                                                                        source_ml_source2_data_property_value12717.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12718 = match builder.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12719, _) => {
                                                                                                            source_ml_source2_data_property_value12719.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12720 = match builder.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12721) => {
                                                                                                                source_ml_source2_data_property_value12721.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12702.clone(), source_ml_source2_nominal_field12704, source_ml_source2_nominal_field12706, source_ml_source2_nominal_field12708, source_ml_source2_nominal_field12710, self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 114, 101, 115, 111, 108, 118, 101, 100, 32, 99, 111, 110, 116, 114, 111, 108, 32, 116, 114, 97, 110, 115, 102, 101, 114, 115]), source_ml_source2_nominal_field12714.clone(), source_ml_source2_nominal_field12716.clone(), source_ml_source2_nominal_field12718.clone(), source_ml_source2_nominal_field12720.clone())
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
                                                                builder_2.clone()
                                                            }
                                                        } else {
                                                            builder.clone()
                                                        };
                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12724, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12724.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![]))) {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12725, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12725.clone()
                                                                }
                                                            }
                                                        } else {
                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()))
                                                        }, if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12727, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12727.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![]))) {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12728, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12728
                                                                }
                                                            }
                                                        } else {
                                                            0f64
                                                        }, match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12729, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12729.clone()
                                                            }
                                                        })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(source_ml_source1_temporary_result59) => {
                                        let source_ml_source1_temporary_result59 = *source_ml_source1_temporary_result59;
                                        source_ml_source1_temporary_result59.clone()
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(source_ml_source1_temporary_result59_2) => {
                        let source_ml_source1_temporary_result59_2 = *source_ml_source1_temporary_result59_2;
                        source_ml_source1_temporary_result59_2.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_control_jump(source: self::JavaScriptCallSource, builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    {
        let target_error = self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 116, 114, 111, 108, 32, 106, 117, 109, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116]);
        match source.clone() {
            self::JavaScriptCallSource::SourceControlJump(target, continuing) => {
                {
                    let scope = { let ml_values142 = { let ml_reference_value143 = ((match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12912, _) => {
                            ((source_ml_source2_data_property_value12912.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())))).borrow().clone(); ml_reference_value143 }; let ml_callback142 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlScope| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12914, _, _) => {
                            source_ml_source2_data_property_value12914
                        }
                    }) == target) }) }; ml_values142.iter().cloned().enumerate().find_map(|(ml_index142, ml_value142)| if ml_callback142(ml_value142.clone()) { Some(ml_value142) } else { None }) };
                    if ((match (scope.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (continuing && (({
                        let source_ml_source2_nullish_field_receiver12918 = (scope.clone(),);
                        if match (source_ml_source2_nullish_field_receiver12918.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present12919 = ((source_ml_source2_nullish_field_receiver12918.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present12919.clone() {
                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, source_ml_source2_data_property_value12920, _) => {
                                        source_ml_source2_data_property_value12920.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 105, 110, 100, 39, 41])).0, message_present: true }).raise()
                        }
                    }) != (self::ml_string::Text::from_units(vec![108, 111, 111, 112]))))) || (({
                        let source_ml_source2_nullish_field_receiver12925 = (scope.clone(),);
                        if match (source_ml_source2_nullish_field_receiver12925.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present12926 = ((source_ml_source2_nullish_field_receiver12925.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present12926.clone() {
                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12927) => {
                                        source_ml_source2_data_property_value12927
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 104, 97, 110, 100, 108, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                        }
                    }) > (match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12928, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12928
                        }
                    }))) {
                        self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107, 32, 97, 110, 100, 32, 99, 111, 110, 116, 105, 110, 117, 101, 32, 114, 101, 116, 97, 105, 110, 32, 97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 97, 110, 100, 32, 104, 97, 110, 100, 108, 101, 114, 32, 100, 101, 112, 116, 104])))
                    } else {
                        {
                            let current = builder.clone();
                            {
                                let position = { let ml_reference_array144 = match current.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12929, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12929.clone()
                                    }
                                }; let ml_reference_projection144 = ml_reference_array144.borrow().len() as f64; ml_reference_projection144 };
                                {
                                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12934, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12934
                                        }
                                    }) > ({
                                        let source_ml_source2_nullish_field_receiver12935 = (scope.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver12935.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source2_nullish_field_present12936 = ((source_ml_source2_nullish_field_receiver12935.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present12936.clone() {
                                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12937) => {
                                                        source_ml_source2_data_property_value12937
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 104, 97, 110, 100, 108, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                                        }
                                    })) {
                                        self::JavaScriptCallInstruction::CompleteJump(0f64, {
                                            let source_ml_source2_nullish_field_receiver12938 = (scope.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver12938.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present12939 = ((source_ml_source2_nullish_field_receiver12938.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present12939.clone() {
                                                        self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12940) => {
                                                            source_ml_source2_data_property_value12940
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 104, 97, 110, 100, 108, 101, 114, 115, 39, 41])).0, message_present: true }).raise()
                                            }
                                        })
                                    } else {
                                        self::JavaScriptCallInstruction::Jump(0f64)
                                    });
                                    {
                                        let transfers = (match current_2.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12941) => {
                                                ((source_ml_source2_data_property_value12941.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new())));
                                        if (({ let ml_reference_array145 = transfers.clone(); let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 }) >= (65536f64)) {
                                            self::refuse_java_script_call_source(current_2.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 110, 116, 114, 111, 108, 32, 116, 114, 97, 110, 115, 102, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 116, 114, 97, 110, 115, 102, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
                                        } else {
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                let source_ml_source2_nominal_field12945 = match current_2.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12946, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12946.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12947 = match current_2.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12948, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12948
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12949 = match current_2.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12950, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12950
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12951 = match current_2.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12952, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12952
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12953 = match current_2.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12954, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12954
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12955 = match current_2.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12956, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12956.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12957 = match current_2.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12958, _, _, _) => {
                                                                                source_ml_source2_data_property_value12958.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12959 = match current_2.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12960, _, _) => {
                                                                                    source_ml_source2_data_property_value12960.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12961 = match current_2.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12962, _) => {
                                                                                        source_ml_source2_data_property_value12962.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12963 = match current_2.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12964) => {
                                                                                            source_ml_source2_data_property_value12964.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override12965 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value12943 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(position, target, continuing)]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlTransfer>::new(), { let ml_reference_value146 = transfers.borrow().clone(); ml_reference_value146 }), { let ml_reference_value147 = source_ml_source2_concat_value12943.borrow().clone(); ml_reference_value147 })
                                                                                        }));
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12945.clone(), source_ml_source2_nominal_field12947, source_ml_source2_nominal_field12949, source_ml_source2_nominal_field12951, source_ml_source2_nominal_field12953, source_ml_source2_nominal_field12955.clone(), source_ml_source2_nominal_field12957.clone(), source_ml_source2_nominal_field12959.clone(), source_ml_source2_nominal_field12961.clone(), Some((Some(source_ml_source2_nominal_override12965.clone()),)))
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
                                            }), 0f64)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
        }
    }
}
