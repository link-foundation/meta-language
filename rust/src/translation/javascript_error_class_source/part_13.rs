// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result;
use super::find_java_script_call_source_instruction_position;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call_source_control_scope(builder: self::JavaScriptCallSourceBuilder, target: f64, kind: &self::ml_string::Text) -> self::JavaScriptCallSourceBuilder {
    {
        let scopes = (match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12966, _) => {
                ((source_ml_source2_data_property_value12966.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())));
        if (((!(self::ml_array::includes(&{ let ml_reference_value148 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 111, 111, 112]), self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104]), self::ml_string::Text::from_units(vec![98, 108, 111, 99, 107])]))).borrow().clone(); ml_reference_value148 }, kind, 0f64, false, |a, b| a == b)) || !(self::test_java_script_call_code_coordinate(target, 65536f64))) || (({ let ml_reference_array149 = scopes.clone(); let ml_reference_projection149 = ml_reference_array149.borrow().len() as f64; ml_reference_projection149 }) >= (128f64))) || { let ml_values150 = { let ml_reference_value151 = scopes.borrow().clone(); ml_reference_value151 }; let ml_callback150 = { let target = target; std::rc::Rc::new(move |scope: self::JavaScriptCallSourceControlScope| -> bool { ((match scope.clone() {
            self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12968, _, _) => {
                source_ml_source2_data_property_value12968
            }
        }) == target) }) }; ml_values150.iter().cloned().enumerate().any(|(ml_index150, ml_value150)| ml_callback150(ml_value150)) }) {
            {
                let source_ml_source2_nominal_field12970 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12971, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12971.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12972 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12973, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12973
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12974 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12975, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12975
                            }
                        };
                        {
                            let source_ml_source2_nominal_field12976 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12977, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12977
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12978 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12979, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12979
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12980 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12981, _, _, _, _) => {
                                            source_ml_source2_data_property_value12981.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12982 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12983, _, _, _) => {
                                                source_ml_source2_data_property_value12983.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12984 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12985, _, _) => {
                                                    source_ml_source2_data_property_value12985.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12986 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12987, _) => {
                                                        source_ml_source2_data_property_value12987.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12988 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12989) => {
                                                            source_ml_source2_data_property_value12989.clone()
                                                        }
                                                    };
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12970.clone(), source_ml_source2_nominal_field12972, source_ml_source2_nominal_field12974, source_ml_source2_nominal_field12976, source_ml_source2_nominal_field12978, self::ml_string::Text::from_units(vec![99, 111, 110, 116, 114, 111, 108, 32, 115, 99, 111, 112, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 119, 105, 116, 104, 105, 110, 32, 116, 104, 101, 105, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 98, 117, 100, 103, 101, 116]), source_ml_source2_nominal_field12982.clone(), source_ml_source2_nominal_field12984.clone(), source_ml_source2_nominal_field12986.clone(), source_ml_source2_nominal_field12988.clone())
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
                let source_ml_source2_nominal_field12995 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12996, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12996.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12997 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12998, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12998
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12999 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13000, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value13000
                            }
                        };
                        {
                            let source_ml_source2_nominal_field13001 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13002, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value13002
                                }
                            };
                            {
                                let source_ml_source2_nominal_field13003 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13004, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value13004
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field13005 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13006, _, _, _, _) => {
                                            source_ml_source2_data_property_value13006.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field13007 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13008, _, _, _) => {
                                                source_ml_source2_data_property_value13008.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field13009 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13010, _, _) => {
                                                    source_ml_source2_data_property_value13010.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field13011 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13012, _) => {
                                                        source_ml_source2_data_property_value13012.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field13013 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13014) => {
                                                            source_ml_source2_data_property_value13014.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override13015 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value12993 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlScope::SourceControlScope(target, kind.clone(), match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12991, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12991
                                                                }
                                                            })]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlScope>::new(), { let ml_reference_value152 = scopes.borrow().clone(); ml_reference_value152 }), { let ml_reference_value153 = source_ml_source2_concat_value12993.borrow().clone(); ml_reference_value153 })
                                                        }));
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12995.clone(), source_ml_source2_nominal_field12997, source_ml_source2_nominal_field12999, source_ml_source2_nominal_field13001, source_ml_source2_nominal_field13003, source_ml_source2_nominal_field13005.clone(), source_ml_source2_nominal_field13007.clone(), source_ml_source2_nominal_field13009.clone(), Some((Some(source_ml_source2_nominal_override13015.clone()),)), source_ml_source2_nominal_field13013.clone())
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
pub fn finish_java_script_call_source_control_scope(builder: self::JavaScriptCallSourceBuilder, outer: self::JavaScriptCallSourceBuilder, target: f64, end: f64, update: f64) -> self::JavaScriptCallSourceBuilder {
    {
        let transfers = (match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13016) => {
                ((source_ml_source2_data_property_value13016.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new())));
        {
            let selected = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values154 = { let ml_reference_value155 = transfers.borrow().clone(); ml_reference_value155 }; let ml_callback154 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { ((match row.clone() {
                self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, source_ml_source2_data_property_value13018, _) => {
                    source_ml_source2_data_property_value13018
                }
            }) == target) }) }; ml_values154.iter().cloned().enumerate().filter_map(|(ml_index154, ml_value154)| if ml_callback154(ml_value154.clone()) { Some(ml_value154) } else { None }).collect::<Vec<_>>() }));
            if { let ml_values156 = { let ml_reference_value157 = selected.borrow().clone(); ml_reference_value157 }; let ml_callback156 = { let update = update; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { (match row.clone() {
                self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, _, source_ml_source2_data_property_value13019) => {
                    source_ml_source2_data_property_value13019
                }
            } && (update < (0f64))) }) }; ml_values156.iter().cloned().enumerate().any(|(ml_index156, ml_value156)| ml_callback156(ml_value156)) } {
                {
                    let source_ml_source2_nominal_field13021 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13022, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value13022.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field13023 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13024, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value13024
                            }
                        };
                        {
                            let source_ml_source2_nominal_field13025 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13026, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value13026
                                }
                            };
                            {
                                let source_ml_source2_nominal_field13027 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13028, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value13028
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field13029 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13030, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value13030
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field13031 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13032, _, _, _, _) => {
                                                source_ml_source2_data_property_value13032.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field13033 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13034, _, _, _) => {
                                                    source_ml_source2_data_property_value13034.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field13035 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13036, _, _) => {
                                                        source_ml_source2_data_property_value13036.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field13037 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13038, _) => {
                                                            source_ml_source2_data_property_value13038.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field13039 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13040) => {
                                                                source_ml_source2_data_property_value13040.clone()
                                                            }
                                                        };
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field13021.clone(), source_ml_source2_nominal_field13023, source_ml_source2_nominal_field13025, source_ml_source2_nominal_field13027, source_ml_source2_nominal_field13029, self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 111, 111, 112, 32, 117, 112, 100, 97, 116, 101, 32, 111, 114, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116]), source_ml_source2_nominal_field13033.clone(), source_ml_source2_nominal_field13035.clone(), source_ml_source2_nominal_field13037.clone(), source_ml_source2_nominal_field13039.clone())
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
                    let positions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values158 = { let ml_reference_value159 = selected.borrow().clone(); ml_reference_value159 }; let ml_callback158 = std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> f64 { match row.clone() {
                        self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(source_ml_source2_data_property_value13042, _, _) => {
                            source_ml_source2_data_property_value13042
                        }
                    } }); ml_values158.iter().cloned().enumerate().map(|(ml_index158, ml_value158)| ml_callback158(ml_value158)).collect::<Vec<_>>() }));
                    {
                        let invalid = {
                            let source_ml_source2_nominal_field13044 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13045, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value13045.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field13046 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13047, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value13047
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field13048 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13049, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value13049
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field13050 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13051, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value13051
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field13052 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13053, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value13053
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field13054 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13055, _, _, _, _) => {
                                                        source_ml_source2_data_property_value13055.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field13056 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13057, _, _, _) => {
                                                            source_ml_source2_data_property_value13057.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field13058 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13059, _, _) => {
                                                                source_ml_source2_data_property_value13059.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field13060 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13061, _) => {
                                                                    source_ml_source2_data_property_value13061.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field13062 = match builder.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13063) => {
                                                                        source_ml_source2_data_property_value13063.clone()
                                                                    }
                                                                };
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field13044.clone(), source_ml_source2_nominal_field13046, source_ml_source2_nominal_field13048, source_ml_source2_nominal_field13050, source_ml_source2_nominal_field13052, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 116, 114, 111, 108, 32, 116, 114, 97, 110, 115, 102, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115]), source_ml_source2_nominal_field13056.clone(), source_ml_source2_nominal_field13058.clone(), source_ml_source2_nominal_field13060.clone(), source_ml_source2_nominal_field13062.clone())
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
                            let previous = (-1f64);
                            {
                                let source_ml_source1_temporary_values60 = positions.clone();
                                {
                                    let source_ml_source1_temporary_index60 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s13065 = self::source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61(builder.clone(), invalid.clone(), previous, source_ml_source1_temporary_values60.clone(), source_ml_source1_temporary_index60.clone());
                                        match source_ml_source2_s13065.clone() {
                                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done => {
                                                {
                                                    let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values160 = { let ml_reference_value161 = (match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13066, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value13066.clone()
                                                        }
                                                    }).borrow().clone(); ml_reference_value161 }; let ml_callback160 = { let positions = positions.clone(); let selected = selected.clone(); let update = update; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { {
                                                        let position = self::find_java_script_call_source_instruction_position(positions.clone(), index);
                                                        if (position < (0f64)) {
                                                            instruction.clone()
                                                        } else {
                                                            {
                                                                let transfer = {
                                                                    let source_ml_source2_array_read_value13067 = selected.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index13068 = position;
                                                                        { let ml_reference_array162 = source_ml_source2_array_read_value13067.clone(); let ml_reference_position162 = source_ml_source2_array_read_index13068; let ml_reference_projection162 = self::ml_array::optional_at(&*ml_reference_array162.borrow(), self::ml_array::number_index(ml_reference_position162)); ml_reference_projection162 }
                                                                    }
                                                                };
                                                                if match (transfer.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    instruction.clone()
                                                                } else {
                                                                    {
                                                                        let destination = if match transfer.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, _, source_ml_source2_data_property_value13069) => {
                                                                                source_ml_source2_data_property_value13069
                                                                            }
                                                                        } {
                                                                            update
                                                                        } else {
                                                                            end
                                                                        };
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
                                                                            self::JavaScriptCallInstruction::CompleteJump(_, depth) => {
                                                                                self::JavaScriptCallInstruction::CompleteJump(destination, depth)
                                                                            }
                                                                            self::JavaScriptCallInstruction::Jump(_) => {
                                                                                self::JavaScriptCallInstruction::Jump(destination)
                                                                            }
                                                                            self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                                                                instruction.clone()
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
                                                    } }) }; ml_values160.iter().cloned().enumerate().map(|(ml_index160, ml_value160)| ml_callback160(ml_value160, (ml_index160 as f64))).collect::<Vec<_>>() }));
                                                    {
                                                        let remaining = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values163 = { let ml_reference_value164 = transfers.borrow().clone(); ml_reference_value164 }; let ml_callback163 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { ((match row.clone() {
                                                            self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, source_ml_source2_data_property_value13317, _) => {
                                                                source_ml_source2_data_property_value13317
                                                            }
                                                        }) != target) }) }; ml_values163.iter().cloned().enumerate().filter_map(|(ml_index163, ml_value163)| if ml_callback163(ml_value163.clone()) { Some(ml_value163) } else { None }).collect::<Vec<_>>() }));
                                                        {
                                                            let source_ml_source2_nominal_field13320 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13321, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value13321.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field13322 = match builder.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13323, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value13323
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field13324 = match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13325, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13325
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field13326 = match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13327, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value13327
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field13328 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13329, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value13329
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field13330 = match builder.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13331, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value13331.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field13332 = match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13333, _, _, _) => {
                                                                                            source_ml_source2_data_property_value13333.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13334 = match builder.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13335, _, _) => {
                                                                                                source_ml_source2_data_property_value13335.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13336 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13337, _) => {
                                                                                                    source_ml_source2_data_property_value13337.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13338 = match builder.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13339) => {
                                                                                                        source_ml_source2_data_property_value13339.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override13341 = (match outer.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13318, _) => {
                                                                                                            ((source_ml_source2_data_property_value13318.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())));
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field13322, source_ml_source2_nominal_field13324, source_ml_source2_nominal_field13326, source_ml_source2_nominal_field13328, source_ml_source2_nominal_field13330.clone(), source_ml_source2_nominal_field13332.clone(), source_ml_source2_nominal_field13334.clone(), Some((Some(source_ml_source2_nominal_override13341.clone()),)), Some((Some(remaining.clone()),)))
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
                                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(source_ml_source1_temporary_result61) => {
                                                let source_ml_source1_temporary_result61 = *source_ml_source1_temporary_result61;
                                                source_ml_source1_temporary_result61.clone()
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
