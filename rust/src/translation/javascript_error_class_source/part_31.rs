// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallArrayPart;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceArrayElement;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result;
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut saved_key: self::JavaScriptCallSourceResult, mut index: self::JavaScriptCallSourceResult, mut source_ml_source1_temporary_values31: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArrayPart>>>, mut source_ml_source1_temporary_index31: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result {
    loop {
        return if (source_ml_source1_temporary_index31 < ({ let ml_reference_array410 = source_ml_source1_temporary_values31.clone(); let ml_reference_projection410 = self::ml::Big::from_u128(ml_reference_array410.borrow().len() as u128); ml_reference_projection410 })) {
            {
                let part = { let ml_reference_array411 = source_ml_source1_temporary_values31.clone(); let ml_reference_position411 = source_ml_source1_temporary_index31.clone(); let ml_reference_projection411 = self::ml_array::at(&*ml_reference_array411.borrow(), ml_reference_position411.to_index()); ml_reference_projection411 };
                if !(self::ml_array::includes(&{ let ml_reference_value412 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 86, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 72, 111, 108, 101])]))).borrow().clone(); ml_reference_value412 }, &match part.clone() {
                    self::JavaScriptCallArrayPart::SourceArrayValue(_) => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 86, 97, 108, 117, 101])
                    }
                    self::JavaScriptCallArrayPart::SourceArraySpread(_) => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100])
                    }
                    self::JavaScriptCallArrayPart::SourceArrayHole => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 72, 111, 108, 101])
                    }
                }, 0f64, false, |a, b| a == b)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 116, 32, 107, 105, 110, 100])))))
                } else {
                    match part.clone() {
                        self::JavaScriptCallArrayPart::SourceArrayValue(source_ml_source2_f18947) => {
                            let source_ml_source2_f18947 = *source_ml_source2_f18947;
                            {
                                let current_3 = {
                                    let key = self::allocate_java_script_call_source_register(current.clone());
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18949, _) => {
                                                let source_ml_source2_data_property_value18949 = *source_ml_source2_data_property_value18949;
                                                source_ml_source2_data_property_value18949.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::PropertyKey(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18950) => {
                                                source_ml_source2_data_property_value18950
                                            }
                                        }, match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18951) => {
                                                source_ml_source2_data_property_value18951
                                            }
                                        }, realm.clone()));
                                        {
                                            let value = self::lower_java_script_call_source_expression(source_ml_source2_f18947.clone(), bindings.clone(), keyed.clone(), (depth + 1f64));
                                            {
                                                let written = self::allocate_java_script_call_source_register(match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18952, _) => {
                                                        let source_ml_source2_data_property_value18952 = *source_ml_source2_data_property_value18952;
                                                        source_ml_source2_data_property_value18952.clone()
                                                    }
                                                });
                                                {
                                                    let current_2 = self::append_java_script_call_source_instruction(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18953, _) => {
                                                            let source_ml_source2_data_property_value18953 = *source_ml_source2_data_property_value18953;
                                                            source_ml_source2_data_property_value18953.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18954) => {
                                                            source_ml_source2_data_property_value18954
                                                        }
                                                    }, match object.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18955) => {
                                                            source_ml_source2_data_property_value18955
                                                        }
                                                    }, match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18956) => {
                                                            source_ml_source2_data_property_value18956
                                                        }
                                                    }, match value.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18957) => {
                                                            source_ml_source2_data_property_value18957
                                                        }
                                                    }, realm.clone()));
                                                    current_2.clone()
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18958) => {
                                            source_ml_source2_data_property_value18958
                                        }
                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18959) => {
                                            source_ml_source2_data_property_value18959
                                        }
                                    }]))));
                                    {
                                        let source_ml_source1_temporary_index31_2 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index31) = (current_4.clone(), source_ml_source1_temporary_index31_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallArrayPart::SourceArraySpread(value_2) => {
                            let value_2 = *value_2;
                            {
                                let input = self::lower_java_script_call_source_expression(value_2.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                {
                                    let saved = self::allocate_java_script_call_source_register(match input.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18962, _) => {
                                            let source_ml_source2_data_property_value18962 = *source_ml_source2_data_property_value18962;
                                            source_ml_source2_data_property_value18962.clone()
                                        }
                                    });
                                    {
                                        let held = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18963, _) => {
                                                let source_ml_source2_data_property_value18963 = *source_ml_source2_data_property_value18963;
                                                source_ml_source2_data_property_value18963.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18964) => {
                                                source_ml_source2_data_property_value18964
                                            }
                                        }, match input.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18965) => {
                                                source_ml_source2_data_property_value18965
                                            }
                                        }));
                                        {
                                            let current_5 = self::append_java_script_call_source_instruction(held.clone(), self::JavaScriptCallInstruction::ArraySpread(match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18966) => {
                                                    source_ml_source2_data_property_value18966
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18967) => {
                                                    source_ml_source2_data_property_value18967
                                                }
                                            }, match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18968) => {
                                                    source_ml_source2_data_property_value18968
                                                }
                                            }, match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18969) => {
                                                    source_ml_source2_data_property_value18969
                                                }
                                            }, match saved_key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18970) => {
                                                    source_ml_source2_data_property_value18970
                                                }
                                            }, realm.clone()));
                                            {
                                                let source_ml_source1_temporary_index31_3 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (current, source_ml_source1_temporary_index31) = (current_5.clone(), source_ml_source1_temporary_index31_3.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallArrayPart::SourceArrayHole => {
                            {
                                let current_6 = current.clone();
                                {
                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18972) => {
                                            source_ml_source2_data_property_value18972
                                        }
                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18973) => {
                                            source_ml_source2_data_property_value18973
                                        }
                                    }]))));
                                    {
                                        let key_2 = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))), bindings.clone(), current_7.clone(), (depth + 1f64));
                                        {
                                            let written_2 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18976, _) => {
                                                    let source_ml_source2_data_property_value18976 = *source_ml_source2_data_property_value18976;
                                                    source_ml_source2_data_property_value18976.clone()
                                                }
                                            });
                                            {
                                                let current_8 = self::append_java_script_call_source_instruction(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18977, _) => {
                                                        let source_ml_source2_data_property_value18977 = *source_ml_source2_data_property_value18977;
                                                        source_ml_source2_data_property_value18977.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::OrdinarySet(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18978) => {
                                                        source_ml_source2_data_property_value18978
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18979) => {
                                                        source_ml_source2_data_property_value18979
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18980) => {
                                                        source_ml_source2_data_property_value18980
                                                    }
                                                }, match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18981) => {
                                                        source_ml_source2_data_property_value18981
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18982) => {
                                                        source_ml_source2_data_property_value18982
                                                    }
                                                }));
                                                {
                                                    let current_9 = self::append_java_script_call_source_instruction(current_8.clone(), self::JavaScriptCallInstruction::RequireWrite(match written_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18983) => {
                                                            source_ml_source2_data_property_value18983
                                                        }
                                                    }));
                                                    {
                                                        let source_ml_source1_temporary_index31_4 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (current, source_ml_source1_temporary_index31) = (current_9.clone(), source_ml_source1_temporary_index31_4.clone());
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result {
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut length: f64, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut previous: f64, mut source_ml_source1_temporary_values32: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, mut source_ml_source1_temporary_index32: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result {
    loop {
        return if (source_ml_source1_temporary_index32 < ({ let ml_reference_array413 = source_ml_source1_temporary_values32.clone(); let ml_reference_projection413 = self::ml::Big::from_u128(ml_reference_array413.borrow().len() as u128); ml_reference_projection413 })) {
            {
                let element = { let ml_reference_array414 = source_ml_source1_temporary_values32.clone(); let ml_reference_position414 = source_ml_source1_temporary_index32.clone(); let ml_reference_projection414 = self::ml_array::at(&*ml_reference_array414.borrow(), ml_reference_position414.to_index()); ml_reference_projection414 };
                if (((!(self::ml_math::is_safe_integer(match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18986, _) => {
                        source_ml_source2_data_property_value18986
                    }
                })) || {
                    let source_ml_source2_same_value_operand18988 = match element.clone() {
                        self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18987, _) => {
                            source_ml_source2_data_property_value18987
                        }
                    };
                    {
                        let source_ml_source2_same_value_operand18989 = (-0f64);
                        (((source_ml_source2_same_value_operand18988 == source_ml_source2_same_value_operand18989) && ((source_ml_source2_same_value_operand18988 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand18988)) == ((1f64 / source_ml_source2_same_value_operand18989))))) || ((source_ml_source2_same_value_operand18988 != source_ml_source2_same_value_operand18988) && (source_ml_source2_same_value_operand18989 != source_ml_source2_same_value_operand18989)))
                    }
                }) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18991, _) => {
                        source_ml_source2_data_property_value18991
                    }
                }) <= previous)) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18993, _) => {
                        source_ml_source2_data_property_value18993
                    }
                }) >= length)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 65, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 105, 110, 99, 114, 101, 97, 115, 105, 110, 103, 32, 108, 105, 116, 101, 114, 97, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])))))
                } else {
                    {
                        let previous_2 = match element.clone() {
                            self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18994, _) => {
                                source_ml_source2_data_property_value18994
                            }
                        };
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Number(match element.clone() {
                                self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value18995, _) => {
                                    source_ml_source2_data_property_value18995
                                }
                            })))), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(match element.clone() {
                                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(_, source_ml_source2_data_property_value18996) => {
                                        let source_ml_source2_data_property_value18996 = *source_ml_source2_data_property_value18996;
                                        source_ml_source2_data_property_value18996.clone()
                                    }
                                }, bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18997, _) => {
                                        let source_ml_source2_data_property_value18997 = *source_ml_source2_data_property_value18997;
                                        source_ml_source2_data_property_value18997.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18998, _) => {
                                            let source_ml_source2_data_property_value18998 = *source_ml_source2_data_property_value18998;
                                            source_ml_source2_data_property_value18998.clone()
                                        }
                                    });
                                    {
                                        let current_2 = self::append_java_script_call_source_instruction(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18999, _) => {
                                                let source_ml_source2_data_property_value18999 = *source_ml_source2_data_property_value18999;
                                                source_ml_source2_data_property_value18999.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19000) => {
                                                source_ml_source2_data_property_value19000
                                            }
                                        }, match object.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19001) => {
                                                source_ml_source2_data_property_value19001
                                            }
                                        }, match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19002) => {
                                                source_ml_source2_data_property_value19002
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19003) => {
                                                source_ml_source2_data_property_value19003
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
        return if (source_ml_source1_temporary_index36 < ({ let ml_reference_array415 = source_ml_source1_temporary_values36.clone(); let ml_reference_projection415 = self::ml::Big::from_u128(ml_reference_array415.borrow().len() as u128); ml_reference_projection415 })) {
            {
                let expression = { let ml_reference_array416 = source_ml_source1_temporary_values36.clone(); let ml_reference_position416 = source_ml_source1_temporary_index36.clone(); let ml_reference_projection416 = self::ml_array::at(&*ml_reference_array416.borrow(), ml_reference_position416.to_index()); ml_reference_projection416 };
                {
                    let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(expression.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match key.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19006, _) => {
                                let source_ml_source2_data_property_value19006 = *source_ml_source2_data_property_value19006;
                                source_ml_source2_data_property_value19006.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19007, _) => {
                                    let source_ml_source2_data_property_value19007 = *source_ml_source2_data_property_value19007;
                                    source_ml_source2_data_property_value19007.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19008) => {
                                    source_ml_source2_data_property_value19008
                                }
                            }, match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19009) => {
                                    source_ml_source2_data_property_value19009
                                }
                            }));
                            {
                                let excluded_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value417 = excluded.borrow().clone(); ml_reference_value417 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19010) => {
                                        source_ml_source2_data_property_value19010
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
        return if (source_ml_source1_temporary_index35 < ({ let ml_reference_array418 = source_ml_source1_temporary_values35.clone(); let ml_reference_projection418 = self::ml::Big::from_u128(ml_reference_array418.borrow().len() as u128); ml_reference_projection418 })) {
            {
                let property = { let ml_reference_array419 = source_ml_source1_temporary_values35.clone(); let ml_reference_position419 = source_ml_source1_temporary_index35.clone(); let ml_reference_projection419 = self::ml_array::at(&*ml_reference_array419.borrow(), ml_reference_position419.to_index()); ml_reference_projection419 };
                match property.clone() {
                    self::JavaScriptCallSourceProperty::SourceObjectProperty(property_key, property_value, infer_name) => {
                        let property_key = *property_key;
                        let property_value = *property_value;
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(property_key.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(property_value.clone(), bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19014, _) => {
                                        let source_ml_source2_data_property_value19014 = *source_ml_source2_data_property_value19014;
                                        source_ml_source2_data_property_value19014.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19015, _) => {
                                            let source_ml_source2_data_property_value19015 = *source_ml_source2_data_property_value19015;
                                            source_ml_source2_data_property_value19015.clone()
                                        }
                                    });
                                    {
                                        let prepared = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19016, _) => {
                                                let source_ml_source2_data_property_value19016 = *source_ml_source2_data_property_value19016;
                                                source_ml_source2_data_property_value19016.clone()
                                            }
                                        };
                                        {
                                            let prepared_3 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named = self::allocate_java_script_call_source_register(prepared.clone());
                                                    {
                                                        let prepared_2 = self::append_java_script_call_source_instruction(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19017, _) => {
                                                                let source_ml_source2_data_property_value19017 = *source_ml_source2_data_property_value19017;
                                                                source_ml_source2_data_property_value19017.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19018) => {
                                                                source_ml_source2_data_property_value19018
                                                            }
                                                        }, match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19019) => {
                                                                source_ml_source2_data_property_value19019
                                                            }
                                                        }, match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19020) => {
                                                                source_ml_source2_data_property_value19020
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19035) => {
                                                        source_ml_source2_data_property_value19035
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19036) => {
                                                        source_ml_source2_data_property_value19036
                                                    }
                                                }, match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19037) => {
                                                        source_ml_source2_data_property_value19037
                                                    }
                                                }, match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19038) => {
                                                        source_ml_source2_data_property_value19038
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19039, _) => {
                                        let source_ml_source2_data_property_value19039 = *source_ml_source2_data_property_value19039;
                                        source_ml_source2_data_property_value19039.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19040, _) => {
                                            let source_ml_source2_data_property_value19040 = *source_ml_source2_data_property_value19040;
                                            source_ml_source2_data_property_value19040.clone()
                                        }
                                    });
                                    {
                                        let prepared_4 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19041, _) => {
                                                let source_ml_source2_data_property_value19041 = *source_ml_source2_data_property_value19041;
                                                source_ml_source2_data_property_value19041.clone()
                                            }
                                        };
                                        {
                                            let prepared_6 = if match ((infer_name_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                {
                                                    let named_2 = self::allocate_java_script_call_source_register(prepared_4.clone());
                                                    {
                                                        let prepared_5 = self::append_java_script_call_source_instruction(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19042, _) => {
                                                                let source_ml_source2_data_property_value19042 = *source_ml_source2_data_property_value19042;
                                                                source_ml_source2_data_property_value19042.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FunctionName(match named_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19043) => {
                                                                source_ml_source2_data_property_value19043
                                                            }
                                                        }, match value_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19044) => {
                                                                source_ml_source2_data_property_value19044
                                                            }
                                                        }, match key_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19045) => {
                                                                source_ml_source2_data_property_value19045
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19056) => {
                                                        source_ml_source2_data_property_value19056
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19057) => {
                                                        source_ml_source2_data_property_value19057
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19058) => {
                                                        source_ml_source2_data_property_value19058
                                                    }
                                                }, match value_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19059) => {
                                                        source_ml_source2_data_property_value19059
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19064, _) => {
                                        let source_ml_source2_data_property_value19064 = *source_ml_source2_data_property_value19064;
                                        source_ml_source2_data_property_value19064.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19065, _) => {
                                            let source_ml_source2_data_property_value19065 = *source_ml_source2_data_property_value19065;
                                            source_ml_source2_data_property_value19065.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::LiteralPrototype(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19066) => {
                                            source_ml_source2_data_property_value19066
                                        }
                                    }, match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19067) => {
                                            source_ml_source2_data_property_value19067
                                        }
                                    }, match value_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19068) => {
                                            source_ml_source2_data_property_value19068
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19069, _) => {
                                        let source_ml_source2_data_property_value19069 = *source_ml_source2_data_property_value19069;
                                        source_ml_source2_data_property_value19069.clone()
                                    }
                                };
                                if (({ let ml_reference_array420 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()))); let ml_reference_projection420 = ml_reference_array420.borrow().len() as f64; ml_reference_projection420 }) > (4096f64)) {
                                    self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(Box::new(self::refuse_java_script_call_source(current_5.clone(), &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 101, 120, 99, 108, 117, 115, 105, 111, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 117, 100, 103, 101, 116])))))
                                } else {
                                    {
                                        let excluded = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                        {
                                            let source_ml_source1_temporary_values36 = (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new())));
                                            {
                                                let source_ml_source1_temporary_index36 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s19070 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42(bindings.clone(), depth, realm.clone(), current_5.clone(), excluded.clone(), source_ml_source1_temporary_values36.clone(), source_ml_source1_temporary_index36.clone());
                                                    match source_ml_source2_s19070.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(current_6, excluded_2) => {
                                                            let current_6 = *current_6;
                                                            {
                                                                let result_4 = self::allocate_java_script_call_source_register(current_6.clone());
                                                                {
                                                                    let current_7 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19071, _) => {
                                                                            let source_ml_source2_data_property_value19071 = *source_ml_source2_data_property_value19071;
                                                                            source_ml_source2_data_property_value19071.clone()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source1_temporary_constructor_field37 = match result_4.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19072) => {
                                                                                source_ml_source2_data_property_value19072
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field38 = match object.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19073) => {
                                                                                    source_ml_source2_data_property_value19073
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field39 = match input.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19074) => {
                                                                                        source_ml_source2_data_property_value19074
                                                                                    }
                                                                                };
                                                                                if !(match (((property_excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field19079 = Some(excluded_2.clone());
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_temporary_constructor_field37, source_ml_source1_temporary_constructor_field38, source_ml_source1_temporary_constructor_field39, realm.clone(), Some((source_ml_source2_constructor_field19079.clone(),)), None::<(Option<self::ml_string::Text>,)>)
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
        return if (source_ml_source1_temporary_index43 < ({ let ml_reference_array421 = source_ml_source1_temporary_values43.clone(); let ml_reference_projection421 = self::ml::Big::from_u128(ml_reference_array421.borrow().len() as u128); ml_reference_projection421 })) {
            {
                let capture = { let ml_reference_array422 = source_ml_source1_temporary_values43.clone(); let ml_reference_position422 = source_ml_source1_temporary_index43.clone(); let ml_reference_projection422 = self::ml_array::at(&*ml_reference_array422.borrow(), ml_reference_position422.to_index()); ml_reference_projection422 };
                {
                    let value = self::lower_java_script_call_source_expression(capture.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19086, _) => {
                                let source_ml_source2_data_property_value19086 = *source_ml_source2_data_property_value19086;
                                source_ml_source2_data_property_value19086.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19087, _) => {
                                    let source_ml_source2_data_property_value19087 = *source_ml_source2_data_property_value19087;
                                    source_ml_source2_data_property_value19087.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19088) => {
                                    source_ml_source2_data_property_value19088
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19089) => {
                                    source_ml_source2_data_property_value19089
                                }
                            }));
                            {
                                let captures_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value423 = captures.borrow().clone(); ml_reference_value423 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19090) => {
                                        source_ml_source2_data_property_value19090
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
        return if (source_ml_source1_temporary_index44 < ({ let ml_reference_array424 = source_ml_source1_temporary_values44.clone(); let ml_reference_projection424 = self::ml::Big::from_u128(ml_reference_array424.borrow().len() as u128); ml_reference_projection424 })) {
            {
                let argument = { let ml_reference_array425 = source_ml_source1_temporary_values44.clone(); let ml_reference_position425 = source_ml_source1_temporary_index44.clone(); let ml_reference_projection425 = self::ml_array::at(&*ml_reference_array425.borrow(), ml_reference_position425.to_index()); ml_reference_projection425 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19093, _) => {
                                let source_ml_source2_data_property_value19093 = *source_ml_source2_data_property_value19093;
                                source_ml_source2_data_property_value19093.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value426 = arguments_.borrow().clone(); ml_reference_value426 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19094) => {
                                    source_ml_source2_data_property_value19094
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
        return if (source_ml_source1_temporary_index47 < ({ let ml_reference_array427 = source_ml_source1_temporary_values47.clone(); let ml_reference_projection427 = self::ml::Big::from_u128(ml_reference_array427.borrow().len() as u128); ml_reference_projection427 })) {
            {
                let argument = { let ml_reference_array428 = source_ml_source1_temporary_values47.clone(); let ml_reference_position428 = source_ml_source1_temporary_index47.clone(); let ml_reference_projection428 = self::ml_array::at(&*ml_reference_array428.borrow(), ml_reference_position428.to_index()); ml_reference_projection428 };
                {
                    let step = self::lower_java_script_call_source_argument(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), slot, match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value19097, _, _, _, _, _) => {
                            source_ml_source2_data_property_value19097
                        }
                    });
                    if ({ let ml_reference_value429 = step.borrow().clone(); ml_reference_value429 }).1 {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 112, 114, 101, 97, 100, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 115, 108, 111, 116, 32, 111, 114, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))))
                    } else {
                        {
                            let current_2 = ({ let ml_reference_value430 = step.borrow().clone(); ml_reference_value430 }).0;
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
