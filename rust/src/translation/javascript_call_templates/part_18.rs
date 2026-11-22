// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result;
use super::find_java_script_call_code_function;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop104(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values88: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index88: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    loop {
        return if source_ml_temporary_index88
            < ({
                let ml_reference_array182 = source_ml_temporary_values88.clone();
                self::ml::Big::from_u128(ml_reference_array182.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array183 = source_ml_temporary_values88.clone();
                    let ml_reference_position183 = source_ml_temporary_index88.clone();
                    self::ml_array::at(
                        &*ml_reference_array183.borrow(),
                        ml_reference_position183.to_index(),
                    )
                };
                {
                    let source_ml_source1_s21031 = match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, immutable_prototype, regular_expression, string_data, primitive_data, callable, _) => {
                            if {
                                let source_ml_source1_comparison_operand21122 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand21123 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand21122, source_ml_source1_comparison_operand21123) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && ((match ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 66, 111, 111, 108, 101, 97, 110]))
                            } else {
                                {
                                    let primitive = ((primitive_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                    {
                                        let text = ((string_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                        if !(match (primitive.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (((callable || !(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])]))).borrow().clone() }, &match primitive.clone().expect("a proven present value") {
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
                                        }, 0f64, false, |a, b| a == b))) || ((self::read_java_script_ordinary_value_error(heap.clone(), primitive.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) || !(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 66, 105, 103, 73, 110, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116]))
                                        } else if !(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((callable || (match text.clone().expect("a proven present value") {
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
                                            })) || ((self::read_java_script_semantic_value_error(text.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 83, 116, 114, 105, 110, 103, 68, 97, 116, 97]))
                                            } else
                                                if {
                                                    let source_ml_source1_comparison_operand21142 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand21143 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand21142, source_ml_source1_comparison_operand21143) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                    }
                                                } && ((match ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 115, 108, 111, 116]))
                                                } else {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                                                }

                                    }
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next
                        }
                    };
                    match source_ml_source1_s21031 {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Next => {
                            if (self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone() }, &match object.clone() {
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
                            }, 0f64, false, |a, b| a == b)) {if ((({
                                    let source_ml_source1_type_query_operand21171 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value21158, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21158.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value21159, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21159.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value21160, _, _, _, _) => {
                                            source_ml_source1_data_property_value21160.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value21161, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21161.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value21162, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21162.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value21163, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21163.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value21164, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21164.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value21165, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21165.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value21166, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21166.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value21167, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21167.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value21168, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21168.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value21169, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21169.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value21170, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value21170.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value21185, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21185.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value21186, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21186.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value21187, _, _, _, _) => {
                                        source_ml_source1_data_property_value21187.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value21188, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21188.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value21189, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21189.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value21190, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21190.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value21191, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21191.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value21192, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21192.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value21193, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21193.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value21194, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21194.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value21195, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21195.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value21196, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21196.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value21197, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21197.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value21211, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21211.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value21212, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21212.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value21213, _, _, _, _) => {
                                        source_ml_source1_data_property_value21213.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value21214, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21214.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value21215, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21215.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value21216, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21216.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value21217, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21217.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value21218, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21218.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value21219, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21219.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value21220, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21220.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value21221, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21221.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value21222, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21222.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value21223, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value21223.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value21224, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21224.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value21225, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21225.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value21226, _, _, _, _) => {
                                                source_ml_source1_data_property_value21226.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value21227, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21227.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value21228, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21228.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value21229, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21229.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value21230, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21230.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value21231, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21231.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value21232, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21232.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value21233, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21233.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value21234, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21234.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value21235, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21235.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value21236, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value21236.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index88_2 = source_ml_temporary_index88.add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_temporary_index88) = (previous_2, source_ml_temporary_index88_2);
                                                continue;
                                            }
                                        }
                                    }
                                }} else {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100]))
                            }
                        }
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin128Return(source_ml_temporary_result128) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_temporary_result128.clone())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallTemplateLoop143Result {
    MlTemporaryFindJavaScriptCallTemplateLoop143Done,
    MlTemporaryFindJavaScriptCallTemplateLoop143Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_template_loop143(
    mut owner: self::ml::Big,
    mut ordinal: f64,
    mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptCallTemplateLoop143Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array186 = templates.clone();
                        let ml_reference_position186 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array186.borrow(),
                            self::ml_array::number_index(ml_reference_position186),
                        )
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptCallTemplateLoop143Result::MlTemporaryFindJavaScriptCallTemplateLoop143Return(-1f64)
                    } else if ((match entry.clone().expect("a proven present value") {
                        self::JavaScriptCallTemplate::CallTemplate(
                            source_ml_source1_data_property_value22285,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22285.clone(),
                    }) == owner)
                        && ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                source_ml_source1_data_property_value22287,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value22287,
                        }) == ordinal)
                    {
                        self::MlTemporaryFindJavaScriptCallTemplateLoop143Result::MlTemporaryFindJavaScriptCallTemplateLoop143Return(middle)
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallTemplate::CallTemplate(
                                source_ml_source1_data_property_value22289,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value22289.clone(),
                        }) < owner)
                            || (((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(
                                    source_ml_source1_data_property_value22291,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value22291.clone(),
                            }) == owner)
                                && ((match entry.clone().expect("a proven present value") {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        source_ml_source1_data_property_value22293,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value22293,
                                }) < ordinal))
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                high = middle;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallTemplateLoop143Result::MlTemporaryFindJavaScriptCallTemplateLoop143Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result {
    MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Done,
    MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(
        Option<self::JavaScriptCallFunction>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_executable_call_function_loop152(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut target: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result {
    loop {
        return if depth
            <= ({
                let ml_reference_array187 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value22295,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value22295.clone(),
                };
                ml_reference_array187.borrow().len() as f64
            }) {
            {
                let current = target.clone();
                match current {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                    }
                    self::JavaScriptSemanticValue::Object(current_identity, callable) => {
                        if callable {
                            {
                                let object = {
                                    let source_ml_source1_array_read_value22305 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value22303, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value22303.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_array_read_index22306 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value22304, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value22304.clone()
                                            }
                                        }, current_identity.clone());
                                        { let ml_reference_array188 = source_ml_source1_array_read_value22305; let ml_reference_position188 = source_ml_source1_array_read_index22306; self::ml_array::optional_at(&*ml_reference_array188.borrow(), self::ml_array::number_index(ml_reference_position188)) }
                                    }
                                };
                                if !(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } {
                                    {
                                        let next = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source1_data_property_value22402, _, _) => {
                                                let source_ml_source1_data_property_value22402 = *source_ml_source1_data_property_value22402;
                                                Some(source_ml_source1_data_property_value22402.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                        };
                                        if match (next.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                                        } else {
                                            {
                                                let target_2 = next.clone().expect("a proven present value");
                                                {
                                                    let depth_2 = depth + 1f64;
                                                    {
                                                        (target, depth) = (target_2, depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let actual = self::read_java_script_executable_call_function(program.clone(), heap.clone(), current_identity.clone());
                                        self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(if match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            None::<self::JavaScriptCallFunction>
                                        } else {
                                            Some({
                                                let source_ml_source1_constructor_field22405 = match actual.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value22404, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value22404.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_constructor_field22407 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_constructor_field22408 = 0f64;
                                                        {
                                                            let source_ml_source1_constructor_field22409 = 2f64;
                                                            {
                                                                let source_ml_source1_constructor_field22410 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                {
                                                                    let source_ml_source1_constructor_field22411 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::BoundFunctionCall(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), source_ml_source1_constructor_field22405, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), Box::new(source_ml_source1_constructor_field22407), source_ml_source1_constructor_field22408, source_ml_source1_constructor_field22409, source_ml_source1_constructor_field22410, source_ml_source1_constructor_field22411, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                        })
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Return(None::<self::JavaScriptCallFunction>)
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Result::MlTemporaryReadJavaScriptExecutableCallFunctionLoop152Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result {
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Done,
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_template_directory_error_loop160(
    mut program: self::JavaScriptCallProgram,
    mut source_ml_temporary_values157: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index157: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result {
    loop {
        return if source_ml_temporary_index157
            < ({
                let ml_reference_array189 = source_ml_temporary_values157.clone();
                self::ml::Big::from_u128(ml_reference_array189.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array190 = source_ml_temporary_values157.clone();
                    let ml_reference_position190 = source_ml_temporary_index157.clone();
                    self::ml_array::at(
                        &*ml_reference_array190.borrow(),
                        ml_reference_position190.to_index(),
                    )
                };
                if match object.clone() {
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                        true
                    }
                    _ => false,
                } && ((self::find_java_script_call_code_function(
                    program.clone(),
                    match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            source_ml_source1_data_property_value22522,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22522.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            source_ml_source1_data_property_value22523,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22523.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            source_ml_source1_data_property_value22524,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22524.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_data_property_value22525,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22525.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            source_ml_source1_data_property_value22526,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22526.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            source_ml_source1_data_property_value22527,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22527.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            source_ml_source1_data_property_value22528,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22528.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            source_ml_source1_data_property_value22529,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22529.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value22530,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22530.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            source_ml_source1_data_property_value22531,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22531.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            source_ml_source1_data_property_value22532,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22532.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            source_ml_source1_data_property_value22533,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22533.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            source_ml_source1_data_property_value22534,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value22534.clone(),
                    },
                )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Return(self::ml_string::Text::from_units(vec![97, 32, 114, 117, 110, 116, 105, 109, 101, 32, 98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 115, 116, 97, 116, 105, 99, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114]))
                } else {
                    {
                        let source_ml_temporary_index157_2 =
                            source_ml_temporary_index157.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index157 = source_ml_temporary_index157_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Result::MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop160Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Result {
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Done,
    MlTemporaryReadJavaScriptCallTemplateDirectoryErrorLoop159Return(self::ml_string::Text),
}
