// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop93(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values77: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index77: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result {
    loop {
        return if source_ml_temporary_index77
            < ({
                let ml_reference_array340 = source_ml_temporary_values77.clone();
                self::ml::Big::from_u128(ml_reference_array340.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array341 = source_ml_temporary_values77.clone();
                    let ml_reference_position341 = source_ml_temporary_index77.clone();
                    self::ml_array::at(
                        &*ml_reference_array341.borrow(),
                        ml_reference_position341.to_index(),
                    )
                };
                {
                    let source_ml_source1_s20293 = match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, immutable_prototype, regular_expression, string_data, primitive_data, callable, _) => {
                            if {
                                let source_ml_source1_comparison_operand20384 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand20385 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand20384, source_ml_source1_comparison_operand20385) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && ((match ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 66, 111, 111, 108, 101, 97, 110]))
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
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 66, 105, 103, 73, 110, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116]))
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
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 83, 116, 114, 105, 110, 103, 68, 97, 116, 97]))
                                            } else
                                                if {
                                                    let source_ml_source1_comparison_operand20404 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand20405 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand20404, source_ml_source1_comparison_operand20405) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                    }
                                                } && ((match ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 115, 108, 111, 116]))
                                                } else {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                                                }

                                    }
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                    };
                    match source_ml_source1_s20293 {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next => {
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
                                    let source_ml_source1_type_query_operand20433 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20420, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20420.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20421, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20421.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20422, _, _, _, _) => {
                                            source_ml_source1_data_property_value20422.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20423, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20423.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20424, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20424.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20425, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20425.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20426, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20426.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20427, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20427.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20428, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20428.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20429, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20429.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20430, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20430.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20431, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20431.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20432, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20432.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20447, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20447.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20448, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20448.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20449, _, _, _, _) => {
                                        source_ml_source1_data_property_value20449.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20450, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20450.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20451, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20451.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20452, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20452.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20453, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20453.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20454, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20454.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20455, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20455.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20456, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20456.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20457, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20457.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20458, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20458.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20459, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20459.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20473, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20473.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20474, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20474.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20475, _, _, _, _) => {
                                        source_ml_source1_data_property_value20475.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20476, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20476.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20477, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20477.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20478, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20478.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20479, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20479.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20480, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20480.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20481, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20481.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20482, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20482.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20483, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20483.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20484, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20484.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20485, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20485.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20486, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20486.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20487, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20487.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20488, _, _, _, _) => {
                                                source_ml_source1_data_property_value20488.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20489, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20489.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20490, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20490.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20491, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20491.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20492, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20492.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20493, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20493.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20494, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20494.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20495, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20495.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20496, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20496.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20497, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20497.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20498, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20498.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index77_2 = source_ml_temporary_index77.add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_temporary_index77) = (previous_2, source_ml_temporary_index77_2);
                                                continue;
                                            }
                                        }
                                    }
                                }} else {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100]))
                            }
                        }
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(source_ml_temporary_result117) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(source_ml_temporary_result117.clone())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_array_own_property_loop120(
    mut count: f64,
    mut source_ml_temporary_values119: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index119: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index119
            < ({
                let ml_reference_array344 = source_ml_temporary_values119.clone();
                self::ml::Big::from_u128(ml_reference_array344.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let ml_reference_array345 = source_ml_temporary_values119.clone();
                    let ml_reference_position345 = source_ml_temporary_index119.clone();
                    self::ml_array::at(
                        &*ml_reference_array345.borrow(),
                        ml_reference_position345.to_index(),
                    )
                };
                {
                    let count_2 = count + {
                        let ml_reference_array346 = match entry {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20560,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20560.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20561,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20561.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20562,
                            ) => source_ml_source1_data_property_value20562.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20563,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20563.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20564,
                                _,
                            ) => source_ml_source1_data_property_value20564.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20565,
                                _,
                            ) => source_ml_source1_data_property_value20565.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20566,
                                _,
                            ) => source_ml_source1_data_property_value20566.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20567,
                                _,
                            ) => source_ml_source1_data_property_value20567.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20568,
                            ) => source_ml_source1_data_property_value20568.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20569,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20569.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20570,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20570.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20571,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20571.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20572,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20572.clone(),
                        };
                        ml_reference_array346.borrow().len() as f64
                    };
                    {
                        let source_ml_temporary_index119_2 =
                            source_ml_temporary_index119.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index119) =
                                (count_2, source_ml_temporary_index119_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { count };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_commit_java_script_ordinary_own_property_loop122(
    mut count: f64,
    mut source_ml_temporary_values121: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index121: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index121
            < ({
                let ml_reference_array347 = source_ml_temporary_values121.clone();
                self::ml::Big::from_u128(ml_reference_array347.borrow().len() as u128)
            })
        {
            {
                let entry = {
                    let ml_reference_array348 = source_ml_temporary_values121.clone();
                    let ml_reference_position348 = source_ml_temporary_index121.clone();
                    self::ml_array::at(
                        &*ml_reference_array348.borrow(),
                        ml_reference_position348.to_index(),
                    )
                };
                {
                    let count_2 = count + {
                        let ml_reference_array349 = match entry {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20575,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20575.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20576,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20576.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20577,
                            ) => source_ml_source1_data_property_value20577.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20578,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20578.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20579,
                                _,
                            ) => source_ml_source1_data_property_value20579.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20580,
                                _,
                            ) => source_ml_source1_data_property_value20580.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20581,
                                _,
                            ) => source_ml_source1_data_property_value20581.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20582,
                                _,
                            ) => source_ml_source1_data_property_value20582.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20583,
                            ) => source_ml_source1_data_property_value20583.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20584,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20584.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20585,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20585.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20586,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20586.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value20587,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20587.clone(),
                        };
                        ml_reference_array349.borrow().len() as f64
                    };
                    {
                        let source_ml_temporary_index121_2 =
                            source_ml_temporary_index121.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index121) =
                                (count_2, source_ml_temporary_index121_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { count };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop129(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values127: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index127: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index127
            < ({
                let ml_reference_array350 = source_ml_temporary_values127.clone();
                self::ml::Big::from_u128(ml_reference_array350.borrow().len() as u128)
            })
        {
            {
                let object = {
                    let ml_reference_array351 = source_ml_temporary_values127.clone();
                    let ml_reference_position351 = source_ml_temporary_index127.clone();
                    self::ml_array::at(
                        &*ml_reference_array351.borrow(),
                        ml_reference_position351.to_index(),
                    )
                };
                if (match object {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        source_ml_source1_data_property_value21529,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21529.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        source_ml_source1_data_property_value21530,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21530.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        source_ml_source1_data_property_value21531,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21531.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        source_ml_source1_data_property_value21532,
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
                    ) => source_ml_source1_data_property_value21532.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        source_ml_source1_data_property_value21533,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21533.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        source_ml_source1_data_property_value21534,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21534.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        source_ml_source1_data_property_value21535,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21535.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        source_ml_source1_data_property_value21536,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21536.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source1_data_property_value21537,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21537.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        source_ml_source1_data_property_value21538,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21538.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        source_ml_source1_data_property_value21539,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21539.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        source_ml_source1_data_property_value21540,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21540.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        source_ml_source1_data_property_value21541,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21541.clone(),
                }) < identity
                {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index127_2 =
                                source_ml_temporary_index127.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index127) =
                                    (at_2, source_ml_temporary_index127_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index127_3 =
                            source_ml_temporary_index127.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index127 = source_ml_temporary_index127_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInsertJavaScriptOrdinaryObjectJoin128 {
    MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next,
    MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Return(
        std::rc::Rc<
            std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>,
        >,
    ),
}
