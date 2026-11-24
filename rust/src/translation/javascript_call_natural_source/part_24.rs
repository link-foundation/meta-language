// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceArrayElement;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptCallSourceSwitchClause;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_argument;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_source_accessor_kind;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut length: f64, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut previous: f64, mut source_ml_source1_temporary_values32: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, mut source_ml_source1_temporary_index32: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result {
    loop {
        return if (source_ml_source1_temporary_index32 < ({ let ml_reference_array259 = source_ml_source1_temporary_values32.clone(); let ml_reference_projection259 = self::ml::Big::from_u128(ml_reference_array259.borrow().len() as u128); ml_reference_projection259 })) {
            {
                let element = { let ml_reference_array260 = source_ml_source1_temporary_values32.clone(); let ml_reference_position260 = source_ml_source1_temporary_index32.clone(); let ml_reference_projection260 = self::ml_array::at(&*ml_reference_array260.borrow(), ml_reference_position260.to_index()); ml_reference_projection260 };
                if (((!(self::ml_math::is_safe_integer(match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15164, _) => {
                        source_ml_source2_data_property_value15164
                    }
                })) || {
                    let source_ml_source2_same_value_operand15166 = match element.clone() {
                        self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15165, _) => {
                            source_ml_source2_data_property_value15165
                        }
                    };
                    {
                        let source_ml_source2_same_value_operand15167 = (-0f64);
                        (((source_ml_source2_same_value_operand15166 == source_ml_source2_same_value_operand15167) && ((source_ml_source2_same_value_operand15166 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand15166)) == ((1f64 / source_ml_source2_same_value_operand15167))))) || ((source_ml_source2_same_value_operand15166 != source_ml_source2_same_value_operand15166) && (source_ml_source2_same_value_operand15167 != source_ml_source2_same_value_operand15167)))
                    }
                }) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15169, _) => {
                        source_ml_source2_data_property_value15169
                    }
                }) <= previous)) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15171, _) => {
                        source_ml_source2_data_property_value15171
                    }
                }) >= length)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 65, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 105, 110, 99, 114, 101, 97, 115, 105, 110, 103, 32, 108, 105, 116, 101, 114, 97, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])))))
                } else {
                    {
                        let previous_2 = match element.clone() {
                            self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15172, _) => {
                                source_ml_source2_data_property_value15172
                            }
                        };
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Number(match element.clone() {
                                self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15173, _) => {
                                    source_ml_source2_data_property_value15173
                                }
                            })))), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(match element.clone() {
                                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(_, source_ml_source2_data_property_value15174) => {
                                        let source_ml_source2_data_property_value15174 = *source_ml_source2_data_property_value15174;
                                        source_ml_source2_data_property_value15174.clone()
                                    }
                                }, bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15175, _) => {
                                        let source_ml_source2_data_property_value15175 = *source_ml_source2_data_property_value15175;
                                        source_ml_source2_data_property_value15175.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15176, _) => {
                                            let source_ml_source2_data_property_value15176 = *source_ml_source2_data_property_value15176;
                                            source_ml_source2_data_property_value15176.clone()
                                        }
                                    });
                                    {
                                        let current_2 = self::append_java_script_call_source_instruction(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15177, _) => {
                                                let source_ml_source2_data_property_value15177 = *source_ml_source2_data_property_value15177;
                                                source_ml_source2_data_property_value15177.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15178) => {
                                                source_ml_source2_data_property_value15178
                                            }
                                        }, match object.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15179) => {
                                                source_ml_source2_data_property_value15179
                                            }
                                        }, match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15180) => {
                                                source_ml_source2_data_property_value15180
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15181) => {
                                                source_ml_source2_data_property_value15181
                                            }
                                        }, realm.clone()));
                                        {
                                            let source_ml_source1_temporary_index32_2 = source_ml_source1_temporary_index32.add(&self::ml::Big::from_i128(1));
                                            {
                                                (current, previous, source_ml_source1_temporary_index32) = (current_2.clone(), previous_2, source_ml_source1_temporary_index32_2.clone());
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result {
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(Box<self::JavaScriptCallSourceResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result {
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut current: self::JavaScriptCallSourceBuilder, mut excluded: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values36: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index36: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result {
    loop {
        return if (source_ml_source1_temporary_index36 < ({ let ml_reference_array261 = source_ml_source1_temporary_values36.clone(); let ml_reference_projection261 = self::ml::Big::from_u128(ml_reference_array261.borrow().len() as u128); ml_reference_projection261 })) {
            {
                let expression = { let ml_reference_array262 = source_ml_source1_temporary_values36.clone(); let ml_reference_position262 = source_ml_source1_temporary_index36.clone(); let ml_reference_projection262 = self::ml_array::at(&*ml_reference_array262.borrow(), ml_reference_position262.to_index()); ml_reference_projection262 };
                {
                    let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(expression.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match key.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15184, _) => {
                                let source_ml_source2_data_property_value15184 = *source_ml_source2_data_property_value15184;
                                source_ml_source2_data_property_value15184.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15185, _) => {
                                    let source_ml_source2_data_property_value15185 = *source_ml_source2_data_property_value15185;
                                    source_ml_source2_data_property_value15185.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15186) => {
                                    source_ml_source2_data_property_value15186
                                }
                            }, match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15187) => {
                                    source_ml_source2_data_property_value15187
                                }
                            }));
                            {
                                let excluded_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value263 = excluded.borrow().clone(); ml_reference_value263 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15188) => {
                                        source_ml_source2_data_property_value15188
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index36_2 = source_ml_source1_temporary_index36.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, excluded, source_ml_source1_temporary_index36) = (current_2.clone(), excluded_2.clone(), source_ml_source1_temporary_index36_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(Box::new(current.clone()), excluded.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut source_ml_source1_temporary_values35: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceProperty>>>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result {
    loop {
        return if (source_ml_source1_temporary_index35 < ({ let ml_reference_array264 = source_ml_source1_temporary_values35.clone(); let ml_reference_projection264 = self::ml::Big::from_u128(ml_reference_array264.borrow().len() as u128); ml_reference_projection264 })) {
            {
                let property = { let ml_reference_array265 = source_ml_source1_temporary_values35.clone(); let ml_reference_position265 = source_ml_source1_temporary_index35.clone(); let ml_reference_projection265 = self::ml_array::at(&*ml_reference_array265.borrow(), ml_reference_position265.to_index()); ml_reference_projection265 };
                match property.clone() {
                    self::JavaScriptCallSourceProperty::SourceObjectProperty(property_key, property_value, infer_name) => {
                        let property_key = *property_key;
                        let property_value = *property_value;
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(property_key.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(property_value.clone(), bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15192, _) => {
                                        let source_ml_source2_data_property_value15192 = *source_ml_source2_data_property_value15192;
                                        source_ml_source2_data_property_value15192.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15193, _) => {
                                            let source_ml_source2_data_property_value15193 = *source_ml_source2_data_property_value15193;
                                            source_ml_source2_data_property_value15193.clone()
                                        }
                                    });
                                    {
                                        let prepared = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15194, _) => {
                                                let source_ml_source2_data_property_value15194 = *source_ml_source2_data_property_value15194;
                                                source_ml_source2_data_property_value15194.clone()
                                            }
                                        };
                                        {
                                            let prepared_3 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named = self::allocate_java_script_call_source_register(prepared.clone());
                                                    {
                                                        let prepared_2 = self::append_java_script_call_source_instruction(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15195, _) => {
                                                                let source_ml_source2_data_property_value15195 = *source_ml_source2_data_property_value15195;
                                                                source_ml_source2_data_property_value15195.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15196) => {
                                                                source_ml_source2_data_property_value15196
                                                            }
                                                        }, match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15197) => {
                                                                source_ml_source2_data_property_value15197
                                                            }
                                                        }, match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15198) => {
                                                                source_ml_source2_data_property_value15198
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15213) => {
                                                        source_ml_source2_data_property_value15213
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15214) => {
                                                        source_ml_source2_data_property_value15214
                                                    }
                                                }, match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15215) => {
                                                        source_ml_source2_data_property_value15215
                                                    }
                                                }, match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15216) => {
                                                        source_ml_source2_data_property_value15216
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15217, _) => {
                                        let source_ml_source2_data_property_value15217 = *source_ml_source2_data_property_value15217;
                                        source_ml_source2_data_property_value15217.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15218, _) => {
                                            let source_ml_source2_data_property_value15218 = *source_ml_source2_data_property_value15218;
                                            source_ml_source2_data_property_value15218.clone()
                                        }
                                    });
                                    {
                                        let prepared_4 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15219, _) => {
                                                let source_ml_source2_data_property_value15219 = *source_ml_source2_data_property_value15219;
                                                source_ml_source2_data_property_value15219.clone()
                                            }
                                        };
                                        {
                                            let prepared_6 = if match ((infer_name_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named_2 = self::allocate_java_script_call_source_register(prepared_4.clone());
                                                    {
                                                        let prepared_5 = self::append_java_script_call_source_instruction(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15220, _) => {
                                                                let source_ml_source2_data_property_value15220 = *source_ml_source2_data_property_value15220;
                                                                source_ml_source2_data_property_value15220.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15221) => {
                                                                source_ml_source2_data_property_value15221
                                                            }
                                                        }, match value_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15222) => {
                                                                source_ml_source2_data_property_value15222
                                                            }
                                                        }, match key_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15223) => {
                                                                source_ml_source2_data_property_value15223
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15234) => {
                                                        source_ml_source2_data_property_value15234
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15235) => {
                                                        source_ml_source2_data_property_value15235
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15236) => {
                                                        source_ml_source2_data_property_value15236
                                                    }
                                                }, match value_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15237) => {
                                                        source_ml_source2_data_property_value15237
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15242, _) => {
                                        let source_ml_source2_data_property_value15242 = *source_ml_source2_data_property_value15242;
                                        source_ml_source2_data_property_value15242.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15243, _) => {
                                            let source_ml_source2_data_property_value15243 = *source_ml_source2_data_property_value15243;
                                            source_ml_source2_data_property_value15243.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::LiteralPrototype(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15244) => {
                                            source_ml_source2_data_property_value15244
                                        }
                                    }, match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15245) => {
                                            source_ml_source2_data_property_value15245
                                        }
                                    }, match value_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15246) => {
                                            source_ml_source2_data_property_value15246
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15247, _) => {
                                        let source_ml_source2_data_property_value15247 = *source_ml_source2_data_property_value15247;
                                        source_ml_source2_data_property_value15247.clone()
                                    }
                                };
                                if (({ let ml_reference_array266 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()))); let ml_reference_projection266 = ml_reference_array266.borrow().len() as f64; ml_reference_projection266 }) > (4096f64)) {
                                    self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(Box::new(self::refuse_java_script_call_source(current_5.clone(), &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 101, 120, 99, 108, 117, 115, 105, 111, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 117, 100, 103, 101, 116])))))
                                } else {
                                    {
                                        let excluded = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                        {
                                            let source_ml_source1_temporary_values36 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new())));
                                            {
                                                let source_ml_source1_temporary_index36 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s15248 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42(bindings.clone(), depth, realm.clone(), current_5.clone(), excluded.clone(), source_ml_source1_temporary_values36.clone(), source_ml_source1_temporary_index36.clone());
                                                    match source_ml_source2_s15248.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(current_6, excluded_2) => {
                                                            let current_6 = *current_6;
                                                            {
                                                                let result_4 = self::allocate_java_script_call_source_register(current_6.clone());
                                                                {
                                                                    let current_7 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15249, _) => {
                                                                            let source_ml_source2_data_property_value15249 = *source_ml_source2_data_property_value15249;
                                                                            source_ml_source2_data_property_value15249.clone()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_temporary_constructor_field37 = match result_4.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15250) => {
                                                                                source_ml_source2_data_property_value15250
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field38 = match object.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15251) => {
                                                                                    source_ml_source2_data_property_value15251
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field39 = match input.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15252) => {
                                                                                        source_ml_source2_data_property_value15252
                                                                                    }
                                                                                };
                                                                                if !(match (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field15257 = Some(excluded_2.clone());
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_temporary_constructor_field37, source_ml_source1_temporary_constructor_field38, source_ml_source1_temporary_constructor_field39, realm.clone(), Some((source_ml_source2_constructor_field15257.clone(),)), None::<(Option<self::ml_string::Text>,)>)
                                                                                    }
                                                                                } else {
                                                                                    self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_temporary_constructor_field37, source_ml_source1_temporary_constructor_field38, source_ml_source1_temporary_constructor_field39, realm.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<self::ml_string::Text>,)>)
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
        return if (source_ml_source1_temporary_index43 < ({ let ml_reference_array267 = source_ml_source1_temporary_values43.clone(); let ml_reference_projection267 = self::ml::Big::from_u128(ml_reference_array267.borrow().len() as u128); ml_reference_projection267 })) {
            {
                let capture = { let ml_reference_array268 = source_ml_source1_temporary_values43.clone(); let ml_reference_position268 = source_ml_source1_temporary_index43.clone(); let ml_reference_projection268 = self::ml_array::at(&*ml_reference_array268.borrow(), ml_reference_position268.to_index()); ml_reference_projection268 };
                {
                    let value = self::lower_java_script_call_source_expression(capture.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15264, _) => {
                                let source_ml_source2_data_property_value15264 = *source_ml_source2_data_property_value15264;
                                source_ml_source2_data_property_value15264.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15265, _) => {
                                    let source_ml_source2_data_property_value15265 = *source_ml_source2_data_property_value15265;
                                    source_ml_source2_data_property_value15265.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15266) => {
                                    source_ml_source2_data_property_value15266
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15267) => {
                                    source_ml_source2_data_property_value15267
                                }
                            }));
                            {
                                let captures_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value269 = captures.borrow().clone(); ml_reference_value269 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15268) => {
                                        source_ml_source2_data_property_value15268
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
        return if (source_ml_source1_temporary_index44 < ({ let ml_reference_array270 = source_ml_source1_temporary_values44.clone(); let ml_reference_projection270 = self::ml::Big::from_u128(ml_reference_array270.borrow().len() as u128); ml_reference_projection270 })) {
            {
                let argument = { let ml_reference_array271 = source_ml_source1_temporary_values44.clone(); let ml_reference_position271 = source_ml_source1_temporary_index44.clone(); let ml_reference_projection271 = self::ml_array::at(&*ml_reference_array271.borrow(), ml_reference_position271.to_index()); ml_reference_projection271 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15271, _) => {
                                let source_ml_source2_data_property_value15271 = *source_ml_source2_data_property_value15271;
                                source_ml_source2_data_property_value15271.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value272 = arguments_.borrow().clone(); ml_reference_value272 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15272) => {
                                    source_ml_source2_data_property_value15272
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
        return if (source_ml_source1_temporary_index47 < ({ let ml_reference_array273 = source_ml_source1_temporary_values47.clone(); let ml_reference_projection273 = self::ml::Big::from_u128(ml_reference_array273.borrow().len() as u128); ml_reference_projection273 })) {
            {
                let argument = { let ml_reference_array274 = source_ml_source1_temporary_values47.clone(); let ml_reference_position274 = source_ml_source1_temporary_index47.clone(); let ml_reference_projection274 = self::ml_array::at(&*ml_reference_array274.borrow(), ml_reference_position274.to_index()); ml_reference_projection274 };
                {
                    let step = self::lower_java_script_call_source_argument(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), slot, match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value15275, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15275
                        }
                    });
                    if ({ let ml_reference_value275 = step.borrow().clone(); ml_reference_value275 }).1 {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 112, 114, 101, 97, 100, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 115, 108, 111, 116, 32, 111, 114, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))))
                    } else {
                        {
                            let current_2 = ({ let ml_reference_value276 = step.borrow().clone(); ml_reference_value276 }).0;
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
                if !({ let ml_values277 = { let ml_reference_value278 = instructions.borrow().clone(); ml_reference_value278 }; let ml_callback277 = { let candidate = candidate; std::rc::Rc::new(move |row: self::JavaScriptCallInstruction| -> bool { match row.clone() {
                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, slot, _, _, _) => {
                        (slot == candidate)
                    }
                    _ => {
                        false
                    }
                } }) }; ml_values277.iter().cloned().enumerate().any(|(ml_index277, ml_value277)| ml_callback277(ml_value277)) }) {
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
        return if (index < ({ let ml_reference_array279 = captures.clone(); let ml_reference_projection279 = ml_reference_array279.borrow().len() as f64; ml_reference_projection279 })) {
            {
                let owner = ({ let ml_reference_array280 = captures.clone(); let ml_reference_position280 = index; let ml_reference_projection280 = self::ml_array::optional_at(&*ml_reference_array280.borrow(), self::ml_array::number_index(ml_reference_position280)); ml_reference_projection280 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || self::ml_array::includes(&{ let ml_reference_value281 = parameters.borrow().clone(); ml_reference_value281 }, &owner, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value282 = captures.borrow().clone(); ml_reference_value282 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115]))))
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
        return if (index < ({ let ml_reference_array283 = parameters.clone(); let ml_reference_projection283 = ml_reference_array283.borrow().len() as f64; ml_reference_projection283 })) {
            {
                let owner = ({ let ml_reference_array284 = parameters.clone(); let ml_reference_position284 = index; let ml_reference_projection284 = self::ml_array::optional_at(&*ml_reference_array284.borrow(), self::ml_array::number_index(ml_reference_position284)); ml_reference_projection284 }).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value285 = parameters.borrow().clone(); ml_reference_value285 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115]))))
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
        return if (source_ml_source1_temporary_index60 < ({ let ml_reference_array286 = source_ml_source1_temporary_values60.clone(); let ml_reference_projection286 = self::ml::Big::from_u128(ml_reference_array286.borrow().len() as u128); ml_reference_projection286 })) {
            {
                let position = { let ml_reference_array287 = source_ml_source1_temporary_values60.clone(); let ml_reference_position287 = source_ml_source1_temporary_index60.clone(); let ml_reference_projection287 = self::ml_array::at(&*ml_reference_array287.borrow(), ml_reference_position287.to_index()); ml_reference_projection287 };
                if (!(self::test_java_script_call_code_coordinate(position, { let ml_reference_array288 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value15528, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value15528.clone()
                    }
                }; let ml_reference_projection288 = ml_reference_array288.borrow().len() as f64; ml_reference_projection288 })) || (position <= previous)) {
                    self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box::new(invalid.clone()))
                } else {
                    {
                        let instruction = ({
                            let source_ml_source2_array_read_value15530 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value15529, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value15529.clone()
                                }
                            };
                            { let ml_reference_array289 = source_ml_source2_array_read_value15530.clone(); let ml_reference_position289 = position; let ml_reference_projection289 = self::ml_array::optional_at(&*ml_reference_array289.borrow(), self::ml_array::number_index(ml_reference_position289)); ml_reference_projection289 }
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
    if (source_ml_source1_temporary_index62 < ({ let ml_reference_array290 = source_ml_source1_temporary_values62.clone(); let ml_reference_projection290 = self::ml::Big::from_u128(ml_reference_array290.borrow().len() as u128); ml_reference_projection290 })) {
        {
            let clause = { let ml_reference_array291 = source_ml_source1_temporary_values62.clone(); let ml_reference_position291 = source_ml_source1_temporary_index62.clone(); let ml_reference_projection291 = self::ml_array::at(&*ml_reference_array291.borrow(), ml_reference_position291.to_index()); ml_reference_projection291 };
            { let ml_cell292 = bodies.clone(); let ml_value292 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value293 = ({ let ml_cell_value = bodies.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value293 }, vec![{ let ml_reference_array294 = match current.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value16028, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value16028.clone()
                }
            }; let ml_reference_projection294 = ml_reference_array294.borrow().len() as f64; ml_reference_projection294 }]))); *ml_cell292.borrow_mut() = ml_value292; {
                let current_2 = match self::lower_java_script_call_source_expression(match clause.clone() {
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(_, source_ml_source2_data_property_value16029) => {
                        let source_ml_source2_data_property_value16029 = *source_ml_source2_data_property_value16029;
                        source_ml_source2_data_property_value16029.clone()
                    }
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(source_ml_source2_data_property_value16030) => {
                        let source_ml_source2_data_property_value16030 = *source_ml_source2_data_property_value16030;
                        source_ml_source2_data_property_value16030.clone()
                    }
                }, bindings.clone(), current.clone(), (depth + 1f64)) {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16031, _) => {
                        let source_ml_source2_data_property_value16031 = *source_ml_source2_data_property_value16031;
                        source_ml_source2_data_property_value16031.clone()
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
