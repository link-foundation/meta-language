// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop93(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_source1_temporary_values77: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index77: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result {
    loop {
        return if (source_ml_source1_temporary_index77
            < ({
                let ml_reference_array2616 = source_ml_source1_temporary_values77.clone();
                let ml_reference_projection2616 =
                    self::ml::Big::from_u128(ml_reference_array2616.borrow().len() as u128);
                ml_reference_projection2616
            })) {
            {
                let object = {
                    let ml_reference_array2617 = source_ml_source1_temporary_values77.clone();
                    let ml_reference_position2617 = source_ml_source1_temporary_index77.clone();
                    let ml_reference_projection2617 = self::ml_array::at(
                        &*ml_reference_array2617.borrow(),
                        ml_reference_position2617.to_index(),
                    );
                    ml_reference_projection2617
                };
                {
                    let source_ml_source2_s39506 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, immutable_prototype, regular_expression, string_data, primitive_data, callable, _) => {
                            if ({
                                let source_ml_source2_comparison_operand39511 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source2_comparison_operand39512 = None::<()>;
                                    !(match (source_ml_source2_comparison_operand39511.clone(), source_ml_source2_comparison_operand39512.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && ((match ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 66, 111, 111, 108, 101, 97, 110]))
                            } else {
                                {
                                    let primitive = ((primitive_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                    {
                                        let text = ((string_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                        if (!(match (primitive.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (((callable || !(self::ml_array::includes(&{ let ml_reference_value2618 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])]))).borrow().clone(); ml_reference_value2618 }, &match primitive.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                            }
                                        }, 0f64, false, |a, b| a == b))) || ((self::read_java_script_ordinary_value_error(heap.clone(), primitive.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) || !(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 66, 105, 103, 73, 110, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116]))
                                        } else {
                                            if (!(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((callable || (match text.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } && match text.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            })) || ((self::read_java_script_semantic_value_error(text.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 83, 116, 114, 105, 110, 103, 68, 97, 116, 97]))
                                            } else {
                                                if ({
                                                    let source_ml_source2_comparison_operand39516 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source2_comparison_operand39517 = None::<()>;
                                                        !(match (source_ml_source2_comparison_operand39516.clone(), source_ml_source2_comparison_operand39517.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                    }
                                                } && ((match ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 115, 108, 111, 116]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                    };
                    match source_ml_source2_s39506.clone() {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next => {
                            if !(self::ml_array::includes(&{ let ml_reference_value2619 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value2619 }, &match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                }
                            }, 0f64, false, |a, b| a == b)) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100]))
                            } else {
                                if (((({
                                    let source_ml_source2_type_query_operand39545 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value39532, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39532.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value39533, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39533.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value39534, _, _, _, _) => {
                                            source_ml_source2_data_property_value39534.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value39535, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39535.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value39536, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39536.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value39537, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39537.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value39538, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39538.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value39539, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39539.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value39540, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39540.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value39541, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39541.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value39542, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39542.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value39543, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39543.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value39544, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39544.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value39559, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39559.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value39560, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39560.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value39561, _, _, _, _) => {
                                        source_ml_source2_data_property_value39561.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value39562, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39562.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value39563, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39563.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value39564, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39564.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value39565, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39565.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value39566, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39566.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value39567, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39567.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value39568, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39568.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value39569, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39569.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value39570, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39570.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value39571, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39571.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value39585, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39585.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value39586, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39586.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value39587, _, _, _, _) => {
                                        source_ml_source2_data_property_value39587.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value39588, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39588.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value39589, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39589.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value39590, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39590.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value39591, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39591.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value39592, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39592.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value39593, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39593.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value39594, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39594.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value39595, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39595.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value39596, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39596.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value39597, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value39597.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value39598, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39598.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value39599, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39599.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value39600, _, _, _, _) => {
                                                source_ml_source2_data_property_value39600.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value39601, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39601.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value39602, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39602.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value39603, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39603.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value39604, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39604.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value39605, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39605.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value39606, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39606.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value39607, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39607.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value39608, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39608.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value39609, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39609.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value39610, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value39610.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index77_2 = source_ml_source1_temporary_index77.add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_source1_temporary_index77) = (previous_2.clone(), source_ml_source1_temporary_index77_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(source_ml_source1_temporary_result117) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(source_ml_source1_temporary_result117.clone())
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    std::rc::Rc<
                        std::cell::RefCell<(
                            (
                                bool,
                                bool,
                                bool,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                bool,
                                bool,
                            ),
                            Vec<f64>,
                        )>,
                    >,
                    bool,
                    self::ml::Big,
                ),
                Vec<f64>,
            )>,
        >,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop118(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut key: self::JavaScriptSemanticValue,
    mut current: self::ml::Big,
    mut depth: f64,
) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (depth
                    < ({
                        let ml_reference_array2620 = shared_constructor_field1.clone();
                        let ml_reference_projection2620 =
                            ml_reference_array2620.borrow().len() as f64;
                        ml_reference_projection2620
                    }))
                {
                    {
                        let index = self::find_java_script_ordinary_object(
                            shared_constructor_field1.clone(),
                            current.clone(),
                        );
                        {
                            let object = {
                                let source_ml_source2_array_read_value39615 =
                                    shared_constructor_field1.clone();
                                {
                                    let ml_reference_array2621 =
                                        source_ml_source2_array_read_value39615.clone();
                                    let ml_reference_position2621 = index;
                                    let ml_reference_projection2621 = self::ml_array::optional_at(
                                        &*ml_reference_array2621.borrow(),
                                        self::ml_array::number_index(ml_reference_position2621),
                                    );
                                    ml_reference_projection2621
                                }
                            };
                            if match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value39617, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39617.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value39618, _, _, _) => {
                                            source_ml_source2_data_property_value39618.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value39619) => {
                                            source_ml_source2_data_property_value39619.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value39620, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value39620.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value39621, _) => {
                                            source_ml_source2_data_property_value39621.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value39622, _) => {
                                            source_ml_source2_data_property_value39622.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value39623, _) => {
                                            source_ml_source2_data_property_value39623.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value39624, _) => {
                                            source_ml_source2_data_property_value39624.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value39625) => {
                                            source_ml_source2_data_property_value39625.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value39626, _, _) => {
                                            source_ml_source2_data_property_value39626.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value39627, _, _) => {
                                            source_ml_source2_data_property_value39627.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value39628, _, _) => {
                                            source_ml_source2_data_property_value39628.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value39629, _, _) => {
                                            source_ml_source2_data_property_value39629.clone()
                                        }
                                    }, key.clone());
                                    {
                                        let property = {
                                            let source_ml_source2_array_read_value39643 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value39630, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value39630.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value39631, _, _, _) => {
                                                    source_ml_source2_data_property_value39631.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value39632) => {
                                                    source_ml_source2_data_property_value39632.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value39633, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value39633.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value39634, _) => {
                                                    source_ml_source2_data_property_value39634.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value39635, _) => {
                                                    source_ml_source2_data_property_value39635.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value39636, _) => {
                                                    source_ml_source2_data_property_value39636.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value39637, _) => {
                                                    source_ml_source2_data_property_value39637.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value39638) => {
                                                    source_ml_source2_data_property_value39638.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value39639, _, _) => {
                                                    source_ml_source2_data_property_value39639.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value39640, _, _) => {
                                                    source_ml_source2_data_property_value39640.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value39641, _, _) => {
                                                    source_ml_source2_data_property_value39641.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value39642, _, _) => {
                                                    source_ml_source2_data_property_value39642.clone()
                                                }
                                            };
                                            {
                                                let ml_reference_array2622 =
                                                    source_ml_source2_array_read_value39643.clone();
                                                let ml_reference_position2622 = at;
                                                let ml_reference_projection2622 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2622.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2622,
                                                        ),
                                                    );
                                                ml_reference_projection2622
                                            }
                                        };
                                        if !(match (property.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) {
                                            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field39648 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value39647) => {
                                                        source_ml_source2_data_property_value39647.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source3_record_field494 = source_ml_source2_field39648.clone();
                                                    {
                                                        let source_ml_source3_record_field495 = true;
                                                        {
                                                            let source_ml_source3_record_field496 = current.clone();
                                                            ((source_ml_source3_record_field494.clone(), source_ml_source3_record_field495, source_ml_source3_record_field496.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![102f64, 111f64, 117f64, 110f64, 100f64], vec![111f64, 119f64, 110f64, 101f64, 114f64]], vec![1f64, 2f64, 0f64]))
                                                        }
                                                    }
                                                }
                                            })))
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value39649, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39649 = *source_ml_source2_data_property_value39649;
                                                        source_ml_source2_data_property_value39649.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value39650, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39650 = *source_ml_source2_data_property_value39650;
                                                        source_ml_source2_data_property_value39650.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value39651, _, _, _) => {
                                                        let source_ml_source2_data_property_value39651 = *source_ml_source2_data_property_value39651;
                                                        source_ml_source2_data_property_value39651.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value39652, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39652 = *source_ml_source2_data_property_value39652;
                                                        source_ml_source2_data_property_value39652.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value39653, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39653 = *source_ml_source2_data_property_value39653;
                                                        source_ml_source2_data_property_value39653.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value39654, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39654 = *source_ml_source2_data_property_value39654;
                                                        source_ml_source2_data_property_value39654.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value39655, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39655 = *source_ml_source2_data_property_value39655;
                                                        source_ml_source2_data_property_value39655.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value39656, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39656 = *source_ml_source2_data_property_value39656;
                                                        source_ml_source2_data_property_value39656.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value39657, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39657 = *source_ml_source2_data_property_value39657;
                                                        source_ml_source2_data_property_value39657.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value39658, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39658 = *source_ml_source2_data_property_value39658;
                                                        source_ml_source2_data_property_value39658.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value39659, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39659 = *source_ml_source2_data_property_value39659;
                                                        source_ml_source2_data_property_value39659.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value39660, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39660 = *source_ml_source2_data_property_value39660;
                                                        source_ml_source2_data_property_value39660.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value39661, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value39661 = *source_ml_source2_data_property_value39661;
                                                        source_ml_source2_data_property_value39661.clone()
                                                    }
                                                };
                                                match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Object(current_2, _) => {
                                                        {
                                                            let depth_2 = (depth + 1f64);
                                                            {
                                                                (current, depth) = (current_2.clone(), depth_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
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
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_array_own_property_loop120(
    mut count: f64,
    mut source_ml_source1_temporary_values119: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index119: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index119
            < ({
                let ml_reference_array2623 = source_ml_source1_temporary_values119.clone();
                let ml_reference_projection2623 =
                    self::ml::Big::from_u128(ml_reference_array2623.borrow().len() as u128);
                ml_reference_projection2623
            })) {
            {
                let entry = {
                    let ml_reference_array2624 = source_ml_source1_temporary_values119.clone();
                    let ml_reference_position2624 = source_ml_source1_temporary_index119.clone();
                    let ml_reference_projection2624 = self::ml_array::at(
                        &*ml_reference_array2624.borrow(),
                        ml_reference_position2624.to_index(),
                    );
                    ml_reference_projection2624
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array2625 = match entry.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39665,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39665.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39666,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39666.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39667,
                            ) => source_ml_source2_data_property_value39667.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39668,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39668.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39669,
                                _,
                            ) => source_ml_source2_data_property_value39669.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39670,
                                _,
                            ) => source_ml_source2_data_property_value39670.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39671,
                                _,
                            ) => source_ml_source2_data_property_value39671.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39672,
                                _,
                            ) => source_ml_source2_data_property_value39672.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39673,
                            ) => source_ml_source2_data_property_value39673.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39674,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39674.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39675,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39675.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39676,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39676.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39677,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39677.clone(),
                        };
                        let ml_reference_projection2625 =
                            ml_reference_array2625.borrow().len() as f64;
                        ml_reference_projection2625
                    });
                    {
                        let source_ml_source1_temporary_index119_2 =
                            source_ml_source1_temporary_index119.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index119) =
                                (count_2, source_ml_source1_temporary_index119_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}
