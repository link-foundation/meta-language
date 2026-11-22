// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptCallSourceSwitchClause;
use super::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_argument;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::read_java_script_call_source_accessor_kind;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut source_ml_source1_temporary_values35: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceProperty>>>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result {
    loop {
        return if (source_ml_source1_temporary_index35 < ({ let ml_reference_array257 = source_ml_source1_temporary_values35.clone(); let ml_reference_projection257 = self::ml::Big::from_u128(ml_reference_array257.borrow().len() as u128); ml_reference_projection257 })) {
            {
                let property = { let ml_reference_array258 = source_ml_source1_temporary_values35.clone(); let ml_reference_position258 = source_ml_source1_temporary_index35.clone(); let ml_reference_projection258 = self::ml_array::at(&*ml_reference_array258.borrow(), ml_reference_position258.to_index()); ml_reference_projection258 };
                match property.clone() {
                    self::JavaScriptCallSourceProperty::SourceObjectProperty(property_key, property_value, infer_name) => {
                        let property_key = *property_key;
                        let property_value = *property_value;
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(property_key.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(property_value.clone(), bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15186, _) => {
                                        let source_ml_source2_data_property_value15186 = *source_ml_source2_data_property_value15186;
                                        source_ml_source2_data_property_value15186.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15187, _) => {
                                            let source_ml_source2_data_property_value15187 = *source_ml_source2_data_property_value15187;
                                            source_ml_source2_data_property_value15187.clone()
                                        }
                                    });
                                    {
                                        let prepared = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15188, _) => {
                                                let source_ml_source2_data_property_value15188 = *source_ml_source2_data_property_value15188;
                                                source_ml_source2_data_property_value15188.clone()
                                            }
                                        };
                                        {
                                            let prepared_3 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named = self::allocate_java_script_call_source_register(prepared.clone());
                                                    {
                                                        let prepared_2 = self::append_java_script_call_source_instruction(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15189, _) => {
                                                                let source_ml_source2_data_property_value15189 = *source_ml_source2_data_property_value15189;
                                                                source_ml_source2_data_property_value15189.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15190) => {
                                                                source_ml_source2_data_property_value15190
                                                            }
                                                        }, match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15191) => {
                                                                source_ml_source2_data_property_value15191
                                                            }
                                                        }, match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15192) => {
                                                                source_ml_source2_data_property_value15192
                                                            }
                                                        }, self::read_java_script_call_source_accessor_kind(property.clone()), realm.clone()));
                                                        prepared_2.clone()
                                                    }
                                                }
                                            } else {
                                                prepared.clone()
                                            };
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(prepared_3.clone(), self::JavaScriptCallInstruction::OrdinaryDataProperty(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15207) => {
                                                        source_ml_source2_data_property_value15207
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15208) => {
                                                        source_ml_source2_data_property_value15208
                                                    }
                                                }, match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15209) => {
                                                        source_ml_source2_data_property_value15209
                                                    }
                                                }, match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15210) => {
                                                        source_ml_source2_data_property_value15210
                                                    }
                                                }, realm.clone()));
                                                {
                                                    let source_ml_source1_temporary_index35_2 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (current, source_ml_source1_temporary_index35) = (current_2.clone(), source_ml_source1_temporary_index35_2.clone());
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
                    self::JavaScriptCallSourceProperty::SourceObjectAccessor(property_key_2, property_value_2, _, infer_name_2) => {
                        let property_key_2 = *property_key_2;
                        let property_value_2 = *property_value_2;
                        {
                            let key_2 = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(property_key_2.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value_2 = self::lower_java_script_call_source_expression(property_value_2.clone(), bindings.clone(), match key_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15211, _) => {
                                        let source_ml_source2_data_property_value15211 = *source_ml_source2_data_property_value15211;
                                        source_ml_source2_data_property_value15211.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15212, _) => {
                                            let source_ml_source2_data_property_value15212 = *source_ml_source2_data_property_value15212;
                                            source_ml_source2_data_property_value15212.clone()
                                        }
                                    });
                                    {
                                        let prepared_4 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15213, _) => {
                                                let source_ml_source2_data_property_value15213 = *source_ml_source2_data_property_value15213;
                                                source_ml_source2_data_property_value15213.clone()
                                            }
                                        };
                                        {
                                            let prepared_6 = if match ((infer_name_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named_2 = self::allocate_java_script_call_source_register(prepared_4.clone());
                                                    {
                                                        let prepared_5 = self::append_java_script_call_source_instruction(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15214, _) => {
                                                                let source_ml_source2_data_property_value15214 = *source_ml_source2_data_property_value15214;
                                                                source_ml_source2_data_property_value15214.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15215) => {
                                                                source_ml_source2_data_property_value15215
                                                            }
                                                        }, match value_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15216) => {
                                                                source_ml_source2_data_property_value15216
                                                            }
                                                        }, match key_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15217) => {
                                                                source_ml_source2_data_property_value15217
                                                            }
                                                        }, self::read_java_script_call_source_accessor_kind(property.clone()), realm.clone()));
                                                        prepared_5.clone()
                                                    }
                                                }
                                            } else {
                                                prepared_4.clone()
                                            };
                                            {
                                                let current_3 = self::append_java_script_call_source_instruction(prepared_6.clone(), self::JavaScriptCallInstruction::LiteralAccessor(match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15228) => {
                                                        source_ml_source2_data_property_value15228
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15229) => {
                                                        source_ml_source2_data_property_value15229
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15230) => {
                                                        source_ml_source2_data_property_value15230
                                                    }
                                                }, match value_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15231) => {
                                                        source_ml_source2_data_property_value15231
                                                    }
                                                }, self::read_java_script_call_source_accessor_kind(property.clone()), realm.clone()));
                                                {
                                                    let source_ml_source1_temporary_index35_3 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (current, source_ml_source1_temporary_index35) = (current_3.clone(), source_ml_source1_temporary_index35_3.clone());
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
                    self::JavaScriptCallSourceProperty::SourceLiteralPrototype(property_value_3) => {
                        let property_value_3 = *property_value_3;
                        {
                            let value_3 = self::lower_java_script_call_source_expression(property_value_3.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15236, _) => {
                                        let source_ml_source2_data_property_value15236 = *source_ml_source2_data_property_value15236;
                                        source_ml_source2_data_property_value15236.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15237, _) => {
                                            let source_ml_source2_data_property_value15237 = *source_ml_source2_data_property_value15237;
                                            source_ml_source2_data_property_value15237.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::LiteralPrototype(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15238) => {
                                            source_ml_source2_data_property_value15238
                                        }
                                    }, match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15239) => {
                                            source_ml_source2_data_property_value15239
                                        }
                                    }, match value_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15240) => {
                                            source_ml_source2_data_property_value15240
                                        }
                                    }));
                                    {
                                        let source_ml_source1_temporary_index35_4 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index35) = (current_4.clone(), source_ml_source1_temporary_index35_4.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSourceProperty::SourceObjectSpread(property_input, property_excluded) => {
                        let property_input = *property_input;
                        {
                            let input = self::lower_java_script_call_source_expression(property_input.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let current_5 = match input.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15241, _) => {
                                        let source_ml_source2_data_property_value15241 = *source_ml_source2_data_property_value15241;
                                        source_ml_source2_data_property_value15241.clone()
                                    }
                                };
                                if (({ let ml_reference_array259 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()))); let ml_reference_projection259 = ml_reference_array259.borrow().len() as f64; ml_reference_projection259 }) > (4096f64)) {
                                    self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(Box::new(self::refuse_java_script_call_source(current_5.clone(), "object copy exclusions exceed their actual operand budget")))
                                } else {
                                    {
                                        let excluded = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                        {
                                            let source_ml_source1_temporary_values36 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new())));
                                            {
                                                let source_ml_source1_temporary_index36 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s15242 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42(bindings.clone(), depth, realm.clone(), current_5.clone(), excluded.clone(), source_ml_source1_temporary_values36.clone(), source_ml_source1_temporary_index36.clone());
                                                    match source_ml_source2_s15242.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(current_6, excluded_2) => {
                                                            let current_6 = *current_6;
                                                            {
                                                                let result_4 = self::allocate_java_script_call_source_register(current_6.clone());
                                                                {
                                                                    let current_7 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15243, _) => {
                                                                            let source_ml_source2_data_property_value15243 = *source_ml_source2_data_property_value15243;
                                                                            source_ml_source2_data_property_value15243.clone()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_temporary_constructor_field37 = match result_4.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15244) => {
                                                                                source_ml_source2_data_property_value15244
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field38 = match object.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15245) => {
                                                                                    source_ml_source2_data_property_value15245
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field39 = match input.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15246) => {
                                                                                        source_ml_source2_data_property_value15246
                                                                                    }
                                                                                };
                                                                                if !(match (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field15251 = Some(excluded_2.clone());
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_temporary_constructor_field37, source_ml_source1_temporary_constructor_field38, source_ml_source1_temporary_constructor_field39, realm.clone(), Some((source_ml_source2_constructor_field15251.clone(),)), None::<(Option<String>,)>)
                                                                                    }
                                                                                } else {
                                                                                    self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_temporary_constructor_field37, source_ml_source1_temporary_constructor_field38, source_ml_source1_temporary_constructor_field39, realm.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<String>,)>)
                                                                                }
                                                                            }
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_temporary_index35_5 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            (current, source_ml_source1_temporary_index35) = (current_7.clone(), source_ml_source1_temporary_index35_5.clone());
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
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result {
    MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values43: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index43: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result {
    loop {
        return if (source_ml_source1_temporary_index43 < ({ let ml_reference_array260 = source_ml_source1_temporary_values43.clone(); let ml_reference_projection260 = self::ml::Big::from_u128(ml_reference_array260.borrow().len() as u128); ml_reference_projection260 })) {
            {
                let capture = { let ml_reference_array261 = source_ml_source1_temporary_values43.clone(); let ml_reference_position261 = source_ml_source1_temporary_index43.clone(); let ml_reference_projection261 = self::ml_array::at(&*ml_reference_array261.borrow(), ml_reference_position261.to_index()); ml_reference_projection261 };
                {
                    let value = self::lower_java_script_call_source_expression(capture.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15258, _) => {
                                let source_ml_source2_data_property_value15258 = *source_ml_source2_data_property_value15258;
                                source_ml_source2_data_property_value15258.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15259, _) => {
                                    let source_ml_source2_data_property_value15259 = *source_ml_source2_data_property_value15259;
                                    source_ml_source2_data_property_value15259.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15260) => {
                                    source_ml_source2_data_property_value15260
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15261) => {
                                    source_ml_source2_data_property_value15261
                                }
                            }));
                            {
                                let captures_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value262 = captures.borrow().clone(); ml_reference_value262 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15262) => {
                                        source_ml_source2_data_property_value15262
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index43_2 = source_ml_source1_temporary_index43.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, captures, source_ml_source1_temporary_index43) = (current_2.clone(), captures_2.clone(), source_ml_source1_temporary_index43_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Done(Box::new(current.clone()), captures.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result {
    MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut snapshot: bool, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values44: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index44: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result {
    loop {
        return if (source_ml_source1_temporary_index44 < ({ let ml_reference_array263 = source_ml_source1_temporary_values44.clone(); let ml_reference_projection263 = self::ml::Big::from_u128(ml_reference_array263.borrow().len() as u128); ml_reference_projection263 })) {
            {
                let argument = { let ml_reference_array264 = source_ml_source1_temporary_values44.clone(); let ml_reference_position264 = source_ml_source1_temporary_index44.clone(); let ml_reference_projection264 = self::ml_array::at(&*ml_reference_array264.borrow(), ml_reference_position264.to_index()); ml_reference_projection264 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15265, _) => {
                                let source_ml_source2_data_property_value15265 = *source_ml_source2_data_property_value15265;
                                source_ml_source2_data_property_value15265.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value265 = arguments_.borrow().clone(); ml_reference_value265 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15266) => {
                                    source_ml_source2_data_property_value15266
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index44_2 = source_ml_source1_temporary_index44.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index44) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index44_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50 {
    MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(Box<self::JavaScriptCallSourceBuilder>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result {
    MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut builder: self::JavaScriptCallSourceBuilder, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut slot: f64, mut source_ml_source1_temporary_values47: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index47: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result {
    loop {
        return if (source_ml_source1_temporary_index47 < ({ let ml_reference_array266 = source_ml_source1_temporary_values47.clone(); let ml_reference_projection266 = self::ml::Big::from_u128(ml_reference_array266.borrow().len() as u128); ml_reference_projection266 })) {
            {
                let argument = { let ml_reference_array267 = source_ml_source1_temporary_values47.clone(); let ml_reference_position267 = source_ml_source1_temporary_index47.clone(); let ml_reference_projection267 = self::ml_array::at(&*ml_reference_array267.borrow(), ml_reference_position267.to_index()); ml_reference_projection267 };
                {
                    let step = self::lower_java_script_call_source_argument(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), slot, match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value15269, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15269
                        }
                    });
                    if ({ let ml_reference_value268 = step.borrow().clone(); ml_reference_value268 }).1 {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(Box::new(self::refuse_java_script_call_source(current.clone(), "argument spread exceeds its actual iterator slot or handler budget")))
                    } else {
                        {
                            let current_2 = ({ let ml_reference_value269 = step.borrow().clone(); ml_reference_value269 }).0;
                            {
                                let source_ml_source1_temporary_index47_2 = source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index47) = (current_2.clone(), source_ml_source1_temporary_index47_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48 {
    MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box<self::JavaScriptCallSourceBuilder>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51(mut instructions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut iterator: f64) -> f64 {
    loop {
        return if (iterator < (65536f64)) {
            {
                let candidate = iterator;
                if !({ let ml_values270 = { let ml_reference_value271 = instructions.borrow().clone(); ml_reference_value271 }; let ml_callback270 = { let candidate = candidate; std::rc::Rc::new(move |row: self::JavaScriptCallInstruction| -> bool { match row.clone() {
                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, slot, _, _, _) => {
                        (slot == candidate)
                    }
                    _ => {
                        false
                    }
                } }) }; ml_values270.iter().cloned().enumerate().any(|(ml_index270, ml_value270)| ml_callback270(ml_value270)) }) {
                    iterator
                } else {
                    {
                        let iterator_2 = (iterator + 1f64);
                        {
                            iterator = iterator_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            iterator
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result {
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done,
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59(mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result {
    loop {
        return if (index < ({ let ml_reference_array272 = captures.clone(); let ml_reference_projection272 = ml_reference_array272.borrow().len() as f64; ml_reference_projection272 })) {
            {
                let owner = ({ let ml_reference_array273 = captures.clone(); let ml_reference_position273 = index; let ml_reference_projection273 = self::ml_array::optional_at(&*ml_reference_array273.borrow(), self::ml_array::number_index(ml_reference_position273)); ml_reference_projection273 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || self::ml_array::includes(&{ let ml_reference_value274 = parameters.borrow().clone(); ml_reference_value274 }, &owner, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value275 = captures.borrow().clone(); ml_reference_value275 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, String::from("source captures require distinct actual lexical owners"))))
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
            self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result {
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done,
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58(mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result {
    loop {
        return if (index < ({ let ml_reference_array276 = parameters.clone(); let ml_reference_projection276 = ml_reference_array276.borrow().len() as f64; ml_reference_projection276 })) {
            {
                let owner = ({ let ml_reference_array277 = parameters.clone(); let ml_reference_position277 = index; let ml_reference_projection277 = self::ml_array::optional_at(&*ml_reference_array277.borrow(), self::ml_array::number_index(ml_reference_position277)); ml_reference_projection277 }).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value278 = parameters.borrow().clone(); ml_reference_value278 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, String::from("source parameters require distinct actual lexical owners"))))
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
            self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result {
    MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done,
    MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box<self::JavaScriptCallSourceBuilder>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61(mut builder: self::JavaScriptCallSourceBuilder, mut invalid: self::JavaScriptCallSourceBuilder, mut previous: f64, mut source_ml_source1_temporary_values60: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index60: self::ml::Big) -> self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result {
    loop {
        return if (source_ml_source1_temporary_index60 < ({ let ml_reference_array279 = source_ml_source1_temporary_values60.clone(); let ml_reference_projection279 = self::ml::Big::from_u128(ml_reference_array279.borrow().len() as u128); ml_reference_projection279 })) {
            {
                let position = { let ml_reference_array280 = source_ml_source1_temporary_values60.clone(); let ml_reference_position280 = source_ml_source1_temporary_index60.clone(); let ml_reference_projection280 = self::ml_array::at(&*ml_reference_array280.borrow(), ml_reference_position280.to_index()); ml_reference_projection280 };
                if (!(self::test_java_script_call_code_coordinate(position, { let ml_reference_array281 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value15522, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value15522.clone()
                    }
                }; let ml_reference_projection281 = ml_reference_array281.borrow().len() as f64; ml_reference_projection281 })) || (position <= previous)) {
                    self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box::new(invalid.clone()))
                } else {
                    {
                        let instruction = ({
                            let source_ml_source2_array_read_value15524 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value15523, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value15523.clone()
                                }
                            };
                            { let ml_reference_array282 = source_ml_source2_array_read_value15524.clone(); let ml_reference_position282 = position; let ml_reference_projection282 = self::ml_array::optional_at(&*ml_reference_array282.borrow(), self::ml_array::number_index(ml_reference_position282)); ml_reference_projection282 }
                        }).clone().unwrap_or_else(|| self::JavaScriptCallInstruction::Return(0f64));
                        if (match instruction.clone() {
                            self::JavaScriptCallInstruction::Jump(_) => {
                                false
                            }
                            _ => {
                                true
                            }
                        } && match instruction.clone() {
                            self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }) {
                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box::new(invalid.clone()))
                        } else {
                            {
                                let source_ml_source1_temporary_index60_2 = source_ml_source1_temporary_index60.add(&self::ml::Big::from_i128(1));
                                {
                                    (previous, source_ml_source1_temporary_index60) = (position, source_ml_source1_temporary_index60_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, depth: f64, current: self::JavaScriptCallSourceBuilder, bodies: std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>, source_ml_source1_temporary_values62: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, source_ml_source1_temporary_index62: self::ml::Big) -> self::JavaScriptCallSourceBuilder {
    if (source_ml_source1_temporary_index62 < ({ let ml_reference_array283 = source_ml_source1_temporary_values62.clone(); let ml_reference_projection283 = self::ml::Big::from_u128(ml_reference_array283.borrow().len() as u128); ml_reference_projection283 })) {
        {
            let clause = { let ml_reference_array284 = source_ml_source1_temporary_values62.clone(); let ml_reference_position284 = source_ml_source1_temporary_index62.clone(); let ml_reference_projection284 = self::ml_array::at(&*ml_reference_array284.borrow(), ml_reference_position284.to_index()); ml_reference_projection284 };
            { let ml_cell285 = bodies.clone(); let ml_value285 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value286 = ({ let ml_cell_value = bodies.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value286 }, vec![{ let ml_reference_array287 = match current.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value16022, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value16022.clone()
                }
            }; let ml_reference_projection287 = ml_reference_array287.borrow().len() as f64; ml_reference_projection287 }]))); *ml_cell285.borrow_mut() = ml_value285; {
                let current_2 = match self::lower_java_script_call_source_expression(match clause.clone() {
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(_, source_ml_source2_data_property_value16023) => {
                        let source_ml_source2_data_property_value16023 = *source_ml_source2_data_property_value16023;
                        source_ml_source2_data_property_value16023.clone()
                    }
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(source_ml_source2_data_property_value16024) => {
                        let source_ml_source2_data_property_value16024 = *source_ml_source2_data_property_value16024;
                        source_ml_source2_data_property_value16024.clone()
                    }
                }, bindings.clone(), current.clone(), (depth + 1f64)) {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16025, _) => {
                        let source_ml_source2_data_property_value16025 = *source_ml_source2_data_property_value16025;
                        source_ml_source2_data_property_value16025.clone()
                    }
                };
                {
                    let source_ml_source1_temporary_index62_2 = source_ml_source1_temporary_index62.add(&self::ml::Big::from_i128(1));
                    self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings.clone(), depth, current_2.clone(), bodies.clone(), source_ml_source1_temporary_values62.clone(), source_ml_source1_temporary_index62_2.clone())
                }
            } }
        }
    } else {
        current.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result {
    MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Done(Box<self::JavaScriptCallSourceBuilder>, f64),
    MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box<self::JavaScriptCallSourceResult>),
}
