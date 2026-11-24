// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_argument_list;
use super::lower_java_script_call_source_array_expression;
use super::lower_java_script_call_source_binding_read_expression;
use super::lower_java_script_call_source_binding_scope;
use super::lower_java_script_call_source_binding_write_expression;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_call_expression;
use super::lower_java_script_call_source_comparison_expression;
use super::lower_java_script_call_source_constructor_expression;
use super::lower_java_script_call_source_control_jump;
use super::lower_java_script_call_source_exception;
use super::lower_java_script_call_source_finally;
use super::lower_java_script_call_source_iterator_expression;
use super::lower_java_script_call_source_lexical_cell_expression;
use super::lower_java_script_call_source_loop;
use super::lower_java_script_call_source_object_expression;
use super::lower_java_script_call_source_property_expression;
use super::lower_java_script_call_source_property_query_expression;
use super::lower_java_script_call_source_switch;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call_source_control_scope;
use super::read_java_script_call_constant_error;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value743, _, _, _, _) => {
            source_ml_source2_data_property_value743.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![]))) {
        builder.clone()
    } else {
        if (({ let ml_reference_array11 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value745, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value745.clone()
            }
        }; let ml_reference_projection11 = ml_reference_array11.borrow().len() as f64; ml_reference_projection11 }) >= (65536f64)) {
            {
                let source_ml_source2_nominal_field747 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value748, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value748.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field749 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value750, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value750
                        }
                    };
                    {
                        let source_ml_source2_nominal_field751 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value752, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value752
                            }
                        };
                        {
                            let source_ml_source2_nominal_field753 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value754, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value754
                                }
                            };
                            {
                                let source_ml_source2_nominal_field755 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value756, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value756
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field757 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value758, _, _, _, _) => {
                                            source_ml_source2_data_property_value758.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field759 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value760, _, _, _) => {
                                                source_ml_source2_data_property_value760.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field761 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value762, _, _) => {
                                                    source_ml_source2_data_property_value762.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field763 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value764, _) => {
                                                        source_ml_source2_data_property_value764.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field765 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value766) => {
                                                            source_ml_source2_data_property_value766.clone()
                                                        }
                                                    };
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field747.clone(), source_ml_source2_nominal_field749, source_ml_source2_nominal_field751, source_ml_source2_nominal_field753, source_ml_source2_nominal_field755, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]), source_ml_source2_nominal_field759.clone(), source_ml_source2_nominal_field761.clone(), source_ml_source2_nominal_field763.clone(), source_ml_source2_nominal_field765.clone())
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
            {
                let source_ml_source2_nominal_field772 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value773, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value773.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field774 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value775, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value775
                        }
                    };
                    {
                        let source_ml_source2_nominal_field776 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value777, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value777
                            }
                        };
                        {
                            let source_ml_source2_nominal_field778 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value779, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value779
                                }
                            };
                            {
                                let source_ml_source2_nominal_field780 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value781, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value781
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field782 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value783, _, _, _, _) => {
                                            source_ml_source2_data_property_value783.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field784 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value785, _, _, _) => {
                                                source_ml_source2_data_property_value785.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field786 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value787, _, _) => {
                                                    source_ml_source2_data_property_value787.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field788 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value789, _) => {
                                                        source_ml_source2_data_property_value789.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field790 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value791) => {
                                                            source_ml_source2_data_property_value791.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override792 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value769 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value768, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value768.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_concat_value770 = std::rc::Rc::new(std::cell::RefCell::new(vec![instruction.clone()]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), { let ml_reference_value12 = source_ml_source2_concat_value769.borrow().clone(); ml_reference_value12 }), { let ml_reference_value13 = source_ml_source2_concat_value770.borrow().clone(); ml_reference_value13 })
                                                            }
                                                        }));
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override792.clone(), source_ml_source2_nominal_field774, source_ml_source2_nominal_field776, source_ml_source2_nominal_field778, source_ml_source2_nominal_field780, source_ml_source2_nominal_field782.clone(), source_ml_source2_nominal_field784.clone(), source_ml_source2_nominal_field786.clone(), source_ml_source2_nominal_field788.clone(), source_ml_source2_nominal_field790.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value794, _, _, _, _) => {
            source_ml_source2_data_property_value794.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![]))) {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value796, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value796
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_field799 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value800, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value800.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field801 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value802, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value802
                        }
                    };
                    {
                        let source_ml_source2_nominal_field803 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value804, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value804
                            }
                        };
                        {
                            let source_ml_source2_nominal_field805 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value806, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value806
                                }
                            };
                            {
                                let source_ml_source2_nominal_field807 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value808, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value808
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field809 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value810, _, _, _, _) => {
                                            source_ml_source2_data_property_value810.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field811 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value812, _, _, _) => {
                                                source_ml_source2_data_property_value812.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field813 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value814, _, _) => {
                                                    source_ml_source2_data_property_value814.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field815 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value816, _) => {
                                                        source_ml_source2_data_property_value816.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field817 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value818) => {
                                                            source_ml_source2_data_property_value818.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override819 = (match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value797, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value797
                                                            }
                                                        } + 1f64);
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field799.clone(), source_ml_source2_nominal_override819, source_ml_source2_nominal_field803, source_ml_source2_nominal_field805, source_ml_source2_nominal_field807, source_ml_source2_nominal_field809.clone(), source_ml_source2_nominal_field811.clone(), source_ml_source2_nominal_field813.clone(), source_ml_source2_nominal_field815.clone(), source_ml_source2_nominal_field817.clone())
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
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value820, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value820
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value822, _, _, _, _) => {
            source_ml_source2_data_property_value822.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![]))) {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value824, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value824
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116])))
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_field827 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value828, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value828.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field829 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value830, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value830
                            }
                        };
                        {
                            let source_ml_source2_nominal_field831 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value832, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value832
                                }
                            };
                            {
                                let source_ml_source2_nominal_field833 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value834, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value834
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field835 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value836, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value836
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field837 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value838, _, _, _, _) => {
                                                source_ml_source2_data_property_value838.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field839 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value840, _, _, _) => {
                                                    source_ml_source2_data_property_value840.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field841 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value842, _, _) => {
                                                        source_ml_source2_data_property_value842.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field843 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value844, _) => {
                                                            source_ml_source2_data_property_value844.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field845 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value846) => {
                                                                source_ml_source2_data_property_value846.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override847 = (match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value825, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value825
                                                                }
                                                            } + 1f64);
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field827.clone(), source_ml_source2_nominal_field829, source_ml_source2_nominal_override847, source_ml_source2_nominal_field833, source_ml_source2_nominal_field835, source_ml_source2_nominal_field837.clone(), source_ml_source2_nominal_field839.clone(), source_ml_source2_nominal_field841.clone(), source_ml_source2_nominal_field843.clone(), source_ml_source2_nominal_field845.clone())
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
                match source.clone() {
                    self::JavaScriptCallSource::SourceConstant(value) => {
                        let value = *value;
                        {
                            let error = self::read_java_script_call_constant_error(value.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                self::refuse_java_script_call_source(state.clone(), &(error))
                            } else {
                                {
                                    let result = self::allocate_java_script_call_source_register(state.clone());
                                    {
                                        let source_ml_source2_nominal_field1026 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1027, _) => {
                                                let source_ml_source2_data_property_value1027 = *source_ml_source2_data_property_value1027;
                                                source_ml_source2_data_property_value1027.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1028 = match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1029) => {
                                                    source_ml_source2_data_property_value1029
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override1030 = self::append_java_script_call_source_instruction(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1023, _) => {
                                                        let source_ml_source2_data_property_value1023 = *source_ml_source2_data_property_value1023;
                                                        source_ml_source2_data_property_value1023.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1024) => {
                                                        source_ml_source2_data_property_value1024
                                                    }
                                                }, Box::new(value.clone())));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1030.clone()), source_ml_source2_nominal_field1028)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceNewTarget => {
                        {
                            let result_2 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1034 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1035, _) => {
                                        let source_ml_source2_data_property_value1035 = *source_ml_source2_data_property_value1035;
                                        source_ml_source2_data_property_value1035.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1036 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1037) => {
                                            source_ml_source2_data_property_value1037
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1038 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1031, _) => {
                                                let source_ml_source2_data_property_value1031 = *source_ml_source2_data_property_value1031;
                                                source_ml_source2_data_property_value1031.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::ReadNewTarget(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1032) => {
                                                source_ml_source2_data_property_value1032
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1038.clone()), source_ml_source2_nominal_field1036)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceReceiver => {
                        self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), 0f64)
                    }
                    self::JavaScriptCallSource::SourceArgument(index) => {
                        if !(self::test_java_script_call_code_coordinate(index, 1048576f64)) {
                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])))
                        } else {
                            {
                                let result_3 = self::allocate_java_script_call_source_register(state.clone());
                                {
                                    let source_ml_source2_nominal_field1042 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1043, _) => {
                                            let source_ml_source2_data_property_value1043 = *source_ml_source2_data_property_value1043;
                                            source_ml_source2_data_property_value1043.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1044 = match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1045) => {
                                                source_ml_source2_data_property_value1045
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override1046 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1039, _) => {
                                                    let source_ml_source2_data_property_value1039 = *source_ml_source2_data_property_value1039;
                                                    source_ml_source2_data_property_value1039.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Argument(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1040) => {
                                                    source_ml_source2_data_property_value1040
                                                }
                                            }, index));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1046.clone()), source_ml_source2_nominal_field1044)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceDerivedReceiver => {
                        {
                            let result_4 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1235 = match result_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1236, _) => {
                                        let source_ml_source2_data_property_value1236 = *source_ml_source2_data_property_value1236;
                                        source_ml_source2_data_property_value1236.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1237 = match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1238) => {
                                            source_ml_source2_data_property_value1238
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1239 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1047, _) => {
                                                let source_ml_source2_data_property_value1047 = *source_ml_source2_data_property_value1047;
                                                source_ml_source2_data_property_value1047.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::ReadDerivedReceiver(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1232) => {
                                                source_ml_source2_data_property_value1232
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1239.clone()), source_ml_source2_nominal_field1237)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceErrorIntrinsic => {
                        {
                            let result_5 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1428 = match result_5.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1429, _) => {
                                        let source_ml_source2_data_property_value1429 = *source_ml_source2_data_property_value1429;
                                        source_ml_source2_data_property_value1429.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1430 = match result_5.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1431) => {
                                            source_ml_source2_data_property_value1431
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1432 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1240, _) => {
                                                let source_ml_source2_data_property_value1240 = *source_ml_source2_data_property_value1240;
                                                source_ml_source2_data_property_value1240.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::InvokeErrorIntrinsic(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1426) => {
                                                source_ml_source2_data_property_value1426
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1432.clone()), source_ml_source2_nominal_field1430)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceConstruct(callee, arguments) => {
                        let callee = *callee;
                        self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(callee.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), false), bindings.clone(), state.clone(), (depth + 1f64))
                    }
                    self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceThrow(source_value) => {
                        let source_value = *source_value;
                        {
                            let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field1621 = match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1622, _) => {
                                        let source_ml_source2_data_property_value1622 = *source_ml_source2_data_property_value1622;
                                        source_ml_source2_data_property_value1622.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1623 = match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1624) => {
                                            source_ml_source2_data_property_value1624
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1625 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1433, _) => {
                                                let source_ml_source2_data_property_value1433 = *source_ml_source2_data_property_value1433;
                                                source_ml_source2_data_property_value1433.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1618) => {
                                                source_ml_source2_data_property_value1618
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1625.clone()), source_ml_source2_nominal_field1623)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceReturn(source_value_2) => {
                        let source_value_2 = *source_value_2;
                        {
                            let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field1814 = match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1815, _) => {
                                        let source_ml_source2_data_property_value1815 = *source_ml_source2_data_property_value1815;
                                        source_ml_source2_data_property_value1815.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1816 = match value_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1817) => {
                                            source_ml_source2_data_property_value1817
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1818 = self::append_java_script_call_source_instruction(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1626, _) => {
                                                let source_ml_source2_data_property_value1626 = *source_ml_source2_data_property_value1626;
                                                source_ml_source2_data_property_value1626.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Return(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1812) => {
                                                source_ml_source2_data_property_value1812
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1818.clone()), source_ml_source2_nominal_field1816)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_loop(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceControlJump(_, _) => {
                        self::lower_java_script_call_source_control_jump(source.clone(), state.clone())
                    }
                    self::JavaScriptCallSource::SourceControlBlock(target, source_body, after) => {
                        let source_body = *source_body;
                        let after = *after;
                        {
                            let scoped = self::push_java_script_call_source_control_scope(state.clone(), target, &(self::ml_string::Text::from_units(vec![98, 108, 111, 99, 107])));
                            {
                                let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), scoped.clone(), (depth + 1f64));
                                {
                                    let finished = self::finish_java_script_call_source_control_scope(match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1819, _) => {
                                            let source_ml_source2_data_property_value1819 = *source_ml_source2_data_property_value1819;
                                            source_ml_source2_data_property_value1819.clone()
                                        }
                                    }, state.clone(), target, { let ml_reference_array14 = match match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1820, _) => {
                                            let source_ml_source2_data_property_value1820 = *source_ml_source2_data_property_value1820;
                                            source_ml_source2_data_property_value1820.clone()
                                        }
                                    } {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value1821, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1821.clone()
                                        }
                                    }; let ml_reference_projection14 = ml_reference_array14.borrow().len() as f64; ml_reference_projection14 }, (-1f64));
                                    self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished.clone(), (depth + 1f64))
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                        self::lower_java_script_call_source_switch(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                        self::lower_java_script_call_source_binding_scope(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                        self::lower_java_script_call_source_lexical_cell_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                        self::lower_java_script_call_source_lexical_cell_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceFinally(_, _) => {
                        self::lower_java_script_call_source_finally(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceTry(_, _, _) => {
                        self::lower_java_script_call_source_exception(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(_) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLet(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIf(_, _, _) => {
                        self::lower_java_script_call_source_branch(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSameValue(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceRecur(_) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceAllocate(_) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceObject(_, _, _) => {
                        self::lower_java_script_call_source_object_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                        self::lower_java_script_call_source_array_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                        self::lower_java_script_call_source_array_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSet(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceGet(_, _, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceDelete(_, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceHas(_, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceCall(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    _ => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                }
            }
        }
    }
}
