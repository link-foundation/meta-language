// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
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
                let ml_reference_array298 = source_ml_temporary_values77.clone();
                self::ml::Big::from_u128(ml_reference_array298.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array299 = source_ml_temporary_values77.clone();
                    let ml_reference_position299 = source_ml_temporary_index77.clone();
                    self::ml_array::at(
                        &*ml_reference_array299.borrow(),
                        ml_reference_position299.to_index(),
                    )
                };
                {
                    let source_ml_source1_s18707 = match object.clone() {
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
                                let source_ml_source1_comparison_operand18798 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand18799 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand18798, source_ml_source1_comparison_operand18799) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                                                    let source_ml_source1_comparison_operand18818 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand18819 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand18818, source_ml_source1_comparison_operand18819) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                    match source_ml_source1_s18707 {
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
                                    let source_ml_source1_type_query_operand18847 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18834, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18834.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18835, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18835.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18836, _, _, _, _) => {
                                            source_ml_source1_data_property_value18836.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18837, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18837.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18838, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18838.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18839, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18839.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18840, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18840.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18841, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18841.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18842, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18842.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18843, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18843.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18844, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18844.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18845, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18845.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18846, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18846.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18861, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18861.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18862, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18862.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18863, _, _, _, _) => {
                                        source_ml_source1_data_property_value18863.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18864, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18864.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18865, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18865.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18866, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18866.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18867, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18867.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18868, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18868.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18869, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18869.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18870, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18870.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18871, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18871.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18872, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18872.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18873, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18873.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18887, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18887.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18888, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18888.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18889, _, _, _, _) => {
                                        source_ml_source1_data_property_value18889.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18890, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18890.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18891, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18891.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18892, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18892.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18893, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18893.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18894, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18894.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18895, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18895.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18896, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18896.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18897, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18897.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18898, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18898.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18899, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18899.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18900, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18900.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18901, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18901.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18902, _, _, _, _) => {
                                                source_ml_source1_data_property_value18902.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18903, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18903.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18904, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18904.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18905, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18905.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18906, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18906.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18907, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18907.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18908, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18908.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18909, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18909.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18910, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18910.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18911, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18911.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18912, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18912.clone()
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
pub fn source_ml_temporary_allocate_java_script_array_loop134(
    mut count: f64,
    mut source_ml_temporary_values132: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index132: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index132
            < ({
                let ml_reference_array302 = source_ml_temporary_values132.clone();
                self::ml::Big::from_u128(ml_reference_array302.borrow().len() as u128)
            })
        {
            {
                let object = {
                    let ml_reference_array303 = source_ml_temporary_values132.clone();
                    let ml_reference_position303 = source_ml_temporary_index132.clone();
                    self::ml_array::at(
                        &*ml_reference_array303.borrow(),
                        ml_reference_position303.to_index(),
                    )
                };
                {
                    let count_2 = count + {
                        let ml_reference_array304 = match object {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19960,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19960.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19961,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19961.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19962,
                            ) => source_ml_source1_data_property_value19962.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19963,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19963.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19964,
                                _,
                            ) => source_ml_source1_data_property_value19964.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19965,
                                _,
                            ) => source_ml_source1_data_property_value19965.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19966,
                                _,
                            ) => source_ml_source1_data_property_value19966.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19967,
                                _,
                            ) => source_ml_source1_data_property_value19967.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19968,
                            ) => source_ml_source1_data_property_value19968.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19969,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19969.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19970,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19970.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19971,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19971.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value19972,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19972.clone(),
                        };
                        ml_reference_array304.borrow().len() as f64
                    };
                    {
                        let source_ml_temporary_index132_2 =
                            source_ml_temporary_index132.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index132) =
                                (count_2, source_ml_temporary_index132_2);
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
pub fn source_ml_temporary_allocate_java_script_array_loop135(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
    loop {
        if (at
            < ({
                let ml_reference_array305 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value19974,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value19974.clone(),
                };
                ml_reference_array305.borrow().len() as f64
            }))
            && ((({
                let source_ml_source1_optional_chain_receiver19995 = ({
                    let source_ml_source1_array_read_value19993 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value19992,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19992.clone(),
                    };
                    {
                        let ml_reference_array306 = source_ml_source1_array_read_value19993;
                        let ml_reference_position306 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array306.borrow(),
                            self::ml_array::number_index(ml_reference_position306),
                        )
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver19995.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_temporary_optional_chain133 =
                            ((source_ml_source1_optional_chain_receiver19995.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_temporary_optional_chain133 {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value19996,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19996.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value19997,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19997.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value19998,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value19998.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value19999,
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
                            ) => source_ml_source1_data_property_value19999.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value20000,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20000.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value20001,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20001.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value20002,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20002.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value20003,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20003.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value20004,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20004.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value20005,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20005.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value20006,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20006.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value20007,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20007.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value20008,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20008.clone(),
                        }
                    })
                } else {
                    None::<self::ml::Big>
                }
            })
            .clone()
            .unwrap_or_else(|| identity.clone()))
                < identity)
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done,
    MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_array_definition_state_error_loop137(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values136: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index136: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    loop {
        return if source_ml_temporary_index136
            < ({
                let ml_reference_array307 = source_ml_temporary_values136.clone();
                self::ml::Big::from_u128(ml_reference_array307.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array308 = source_ml_temporary_values136.clone();
                    let ml_reference_position308 = source_ml_temporary_index136.clone();
                    self::ml_array::at(
                        &*ml_reference_array308.borrow(),
                        ml_reference_position308.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index136_2 =
                                source_ml_temporary_index136.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index136 = source_ml_temporary_index136_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(invalid.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done
        };
    }
}
