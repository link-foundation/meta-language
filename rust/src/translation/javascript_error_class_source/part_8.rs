// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_property_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceAllocate(source_prototype) => {
            let source_prototype = *source_prototype;
            {
                let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match prototype.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8156, _) => {
                            let source_ml_source2_data_property_value8156 = *source_ml_source2_data_property_value8156;
                            source_ml_source2_data_property_value8156.clone()
                        }
                    });
                    {
                        let source_ml_source2_nominal_field8161 = match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8162, _) => {
                                let source_ml_source2_data_property_value8162 = *source_ml_source2_data_property_value8162;
                                source_ml_source2_data_property_value8162.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field8163 = match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8164) => {
                                    source_ml_source2_data_property_value8164
                                }
                            };
                            {
                                let source_ml_source2_nominal_override8165 = self::append_java_script_call_source_instruction(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8157, _) => {
                                        let source_ml_source2_data_property_value8157 = *source_ml_source2_data_property_value8157;
                                        source_ml_source2_data_property_value8157.clone()
                                    }
                                }, self::JavaScriptCallInstruction::OrdinaryAllocate(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8158) => {
                                        source_ml_source2_data_property_value8158
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8159) => {
                                        source_ml_source2_data_property_value8159
                                    }
                                }));
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8165.clone()), source_ml_source2_nominal_field8163)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourcePropertyKey(source_input, realm) => {
            let source_input = *source_input;
            if ((realm <= (self::ml::Big::from_i128(0))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
            } else {
                {
                    let input = self::lower_java_script_call_source_expression(source_input.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let result_2 = self::allocate_java_script_call_source_register(match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8166, _) => {
                                let source_ml_source2_data_property_value8166 = *source_ml_source2_data_property_value8166;
                                source_ml_source2_data_property_value8166.clone()
                            }
                        });
                        {
                            let source_ml_source2_nominal_field8171 = match result_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8172, _) => {
                                    let source_ml_source2_data_property_value8172 = *source_ml_source2_data_property_value8172;
                                    source_ml_source2_data_property_value8172.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field8173 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8174) => {
                                        source_ml_source2_data_property_value8174
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override8175 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8167, _) => {
                                            let source_ml_source2_data_property_value8167 = *source_ml_source2_data_property_value8167;
                                            source_ml_source2_data_property_value8167.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::PropertyKey(match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8168) => {
                                            source_ml_source2_data_property_value8168
                                        }
                                    }, match input.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8169) => {
                                            source_ml_source2_data_property_value8169
                                        }
                                    }, realm.clone()));
                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8175.clone()), source_ml_source2_nominal_field8173)
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(source_target, source_key, source_attributes, realm_2) => {
            let source_target = *source_target;
            let source_key = *source_key;
            let source_attributes = *source_attributes;
            if ((realm_2 <= (self::ml::Big::from_i128(0))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
            } else {
                {
                    let target = self::lower_java_script_call_source_expression(source_target.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), match target.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8176, _) => {
                                let source_ml_source2_data_property_value8176 = *source_ml_source2_data_property_value8176;
                                source_ml_source2_data_property_value8176.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let attributes = self::lower_java_script_call_source_expression(source_attributes.clone(), bindings.clone(), match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8177, _) => {
                                    let source_ml_source2_data_property_value8177 = *source_ml_source2_data_property_value8177;
                                    source_ml_source2_data_property_value8177.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match attributes.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8178, _) => {
                                        let source_ml_source2_data_property_value8178 = *source_ml_source2_data_property_value8178;
                                        source_ml_source2_data_property_value8178.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field8190 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8191, _) => {
                                            let source_ml_source2_data_property_value8191 = *source_ml_source2_data_property_value8191;
                                            source_ml_source2_data_property_value8191.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field8192 = match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8193) => {
                                                source_ml_source2_data_property_value8193
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override8194 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8179, _) => {
                                                    let source_ml_source2_data_property_value8179 = *source_ml_source2_data_property_value8179;
                                                    source_ml_source2_data_property_value8179.clone()
                                                }
                                            }, {
                                                let source_ml_source2_constructor_field8181 = match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8180) => {
                                                        source_ml_source2_data_property_value8180
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_constructor_field8183 = match target.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8182) => {
                                                            source_ml_source2_data_property_value8182
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field8185 = match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8184) => {
                                                                source_ml_source2_data_property_value8184
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field8187 = match attributes.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8186) => {
                                                                    source_ml_source2_data_property_value8186
                                                                }
                                                            };
                                                            self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_constructor_field8181, source_ml_source2_constructor_field8183, source_ml_source2_constructor_field8185, source_ml_source2_constructor_field8187, realm_2.clone(), None::<(Option<self::ml_string::Text>,)>)
                                                        }
                                                    }
                                                }
                                            });
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8194.clone()), source_ml_source2_nominal_field8192)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSet(source_object, source_key_2, source_value) => {
            let source_object = *source_object;
            let source_key_2 = *source_key_2;
            let source_value = *source_value;
            {
                let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_2 = object.clone();
                    {
                        let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings.clone(), match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8379, _) => {
                                let source_ml_source2_data_property_value8379 = *source_ml_source2_data_property_value8379;
                                source_ml_source2_data_property_value8379.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let key_3 = key_2.clone();
                            {
                                let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8564, _) => {
                                        let source_ml_source2_data_property_value8564 = *source_ml_source2_data_property_value8564;
                                        source_ml_source2_data_property_value8564.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_4 = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8565, _) => {
                                            let source_ml_source2_data_property_value8565 = *source_ml_source2_data_property_value8565;
                                            source_ml_source2_data_property_value8565.clone()
                                        }
                                    });
                                    {
                                        let write = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8566, _) => {
                                                let source_ml_source2_data_property_value8566 = *source_ml_source2_data_property_value8566;
                                                source_ml_source2_data_property_value8566.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinarySet(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8567) => {
                                                source_ml_source2_data_property_value8567
                                            }
                                        }, match object_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8568) => {
                                                source_ml_source2_data_property_value8568
                                            }
                                        }, match key_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8569) => {
                                                source_ml_source2_data_property_value8569
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8570) => {
                                                source_ml_source2_data_property_value8570
                                            }
                                        }, match object_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8571) => {
                                                source_ml_source2_data_property_value8571
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8572) => {
                                                source_ml_source2_data_property_value8572
                                            }
                                        }))), match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8573) => {
                                                source_ml_source2_data_property_value8573
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
        self::JavaScriptCallSource::SourceSnapshotSet(source_object_2, source_key_3, source_value_2) => {
            let source_object_2 = *source_object_2;
            let source_key_3 = *source_key_3;
            let source_value_2 = *source_value_2;
            {
                let object_3 = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_5 = {
                        let saved = self::allocate_java_script_call_source_register(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8758, _) => {
                                let source_ml_source2_data_property_value8758 = *source_ml_source2_data_property_value8758;
                                source_ml_source2_data_property_value8758.clone()
                            }
                        });
                        {
                            let object_4 = {
                                let source_ml_source2_nominal_field8763 = match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8764, _) => {
                                        let source_ml_source2_data_property_value8764 = *source_ml_source2_data_property_value8764;
                                        source_ml_source2_data_property_value8764.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field8765 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8766) => {
                                            source_ml_source2_data_property_value8766
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override8767 = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8759, _) => {
                                                let source_ml_source2_data_property_value8759 = *source_ml_source2_data_property_value8759;
                                                source_ml_source2_data_property_value8759.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8760) => {
                                                source_ml_source2_data_property_value8760
                                            }
                                        }, match object_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8761) => {
                                                source_ml_source2_data_property_value8761
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8767.clone()), source_ml_source2_nominal_field8765)
                                    }
                                }
                            };
                            object_4.clone()
                        }
                    };
                    {
                        let key_4 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), match object_5.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8768, _) => {
                                let source_ml_source2_data_property_value8768 = *source_ml_source2_data_property_value8768;
                                source_ml_source2_data_property_value8768.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let key_6 = {
                                let saved_2 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8953, _) => {
                                        let source_ml_source2_data_property_value8953 = *source_ml_source2_data_property_value8953;
                                        source_ml_source2_data_property_value8953.clone()
                                    }
                                });
                                {
                                    let key_5 = {
                                        let source_ml_source2_nominal_field8958 = match saved_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8959, _) => {
                                                let source_ml_source2_data_property_value8959 = *source_ml_source2_data_property_value8959;
                                                source_ml_source2_data_property_value8959.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field8960 = match saved_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8961) => {
                                                    source_ml_source2_data_property_value8961
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override8962 = self::append_java_script_call_source_instruction(match saved_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8954, _) => {
                                                        let source_ml_source2_data_property_value8954 = *source_ml_source2_data_property_value8954;
                                                        source_ml_source2_data_property_value8954.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Copy(match saved_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8955) => {
                                                        source_ml_source2_data_property_value8955
                                                    }
                                                }, match key_4.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8956) => {
                                                        source_ml_source2_data_property_value8956
                                                    }
                                                }));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8962.clone()), source_ml_source2_nominal_field8960)
                                            }
                                        }
                                    };
                                    key_5.clone()
                                }
                            };
                            {
                                let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), match key_6.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8963, _) => {
                                        let source_ml_source2_data_property_value8963 = *source_ml_source2_data_property_value8963;
                                        source_ml_source2_data_property_value8963.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_5 = self::allocate_java_script_call_source_register(match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8964, _) => {
                                            let source_ml_source2_data_property_value8964 = *source_ml_source2_data_property_value8964;
                                            source_ml_source2_data_property_value8964.clone()
                                        }
                                    });
                                    {
                                        let write_2 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8965, _) => {
                                                let source_ml_source2_data_property_value8965 = *source_ml_source2_data_property_value8965;
                                                source_ml_source2_data_property_value8965.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinarySet(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8966) => {
                                                source_ml_source2_data_property_value8966
                                            }
                                        }, match object_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8967) => {
                                                source_ml_source2_data_property_value8967
                                            }
                                        }, match key_6.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8968) => {
                                                source_ml_source2_data_property_value8968
                                            }
                                        }, match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8969) => {
                                                source_ml_source2_data_property_value8969
                                            }
                                        }, match object_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8970) => {
                                                source_ml_source2_data_property_value8970
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write_2.clone(), self::JavaScriptCallInstruction::RequireWrite(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8971) => {
                                                source_ml_source2_data_property_value8971
                                            }
                                        }))), match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8972) => {
                                                source_ml_source2_data_property_value8972
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
        self::JavaScriptCallSource::SourceInstanceOf(source_left, source_right, source_key_4) => {
            let source_left = *source_left;
            let source_right = *source_right;
            let source_key_4 = *source_key_4;
            {
                let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let held_left = self::allocate_java_script_call_source_register(match left.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8973, _) => {
                            let source_ml_source2_data_property_value8973 = *source_ml_source2_data_property_value8973;
                            source_ml_source2_data_property_value8973.clone()
                        }
                    });
                    {
                        let saved_left = self::append_java_script_call_source_instruction(match held_left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8974, _) => {
                                let source_ml_source2_data_property_value8974 = *source_ml_source2_data_property_value8974;
                                source_ml_source2_data_property_value8974.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held_left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8975) => {
                                source_ml_source2_data_property_value8975
                            }
                        }, match left.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8976) => {
                                source_ml_source2_data_property_value8976
                            }
                        }));
                        {
                            let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings.clone(), saved_left.clone(), (depth + 1f64));
                            {
                                let held_right = self::allocate_java_script_call_source_register(match right.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8977, _) => {
                                        let source_ml_source2_data_property_value8977 = *source_ml_source2_data_property_value8977;
                                        source_ml_source2_data_property_value8977.clone()
                                    }
                                });
                                {
                                    let saved_right = self::append_java_script_call_source_instruction(match held_right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8978, _) => {
                                            let source_ml_source2_data_property_value8978 = *source_ml_source2_data_property_value8978;
                                            source_ml_source2_data_property_value8978.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8979) => {
                                            source_ml_source2_data_property_value8979
                                        }
                                    }, match right.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8980) => {
                                            source_ml_source2_data_property_value8980
                                        }
                                    }));
                                    {
                                        let key_7 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_right.clone(), (depth + 1f64));
                                        {
                                            let result_6 = self::allocate_java_script_call_source_register(match key_7.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8981, _) => {
                                                    let source_ml_source2_data_property_value8981 = *source_ml_source2_data_property_value8981;
                                                    source_ml_source2_data_property_value8981.clone()
                                                }
                                            });
                                            {
                                                let source_ml_source2_nominal_field8988 = match result_6.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8989, _) => {
                                                        let source_ml_source2_data_property_value8989 = *source_ml_source2_data_property_value8989;
                                                        source_ml_source2_data_property_value8989.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field8990 = match result_6.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8991) => {
                                                            source_ml_source2_data_property_value8991
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override8992 = self::append_java_script_call_source_instruction(match result_6.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value8982, _) => {
                                                                let source_ml_source2_data_property_value8982 = *source_ml_source2_data_property_value8982;
                                                                source_ml_source2_data_property_value8982.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::InstanceOf(match result_6.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8983) => {
                                                                source_ml_source2_data_property_value8983
                                                            }
                                                        }, match held_left.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8984) => {
                                                                source_ml_source2_data_property_value8984
                                                            }
                                                        }, match held_right.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8985) => {
                                                                source_ml_source2_data_property_value8985
                                                            }
                                                        }, match key_7.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value8986) => {
                                                                source_ml_source2_data_property_value8986
                                                            }
                                                        }, self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102])));
                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override8992.clone()), source_ml_source2_nominal_field8990)
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
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
