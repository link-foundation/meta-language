// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin117;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::move_java_script_collection_iterator_cursor;
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
                let ml_reference_array180 = source_ml_temporary_values77.clone();
                self::ml::Big::from_u128(ml_reference_array180.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array181 = source_ml_temporary_values77.clone();
                    let ml_reference_position181 = source_ml_temporary_index77.clone();
                    self::ml_array::at(
                        &*ml_reference_array181.borrow(),
                        ml_reference_position181.to_index(),
                    )
                };
                {
                    let source_ml_source1_s20429 = match object.clone() {
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
                                let source_ml_source1_comparison_operand20520 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand20521 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand20520, source_ml_source1_comparison_operand20521) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                                                    let source_ml_source1_comparison_operand20540 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand20541 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand20540, source_ml_source1_comparison_operand20541) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                    match source_ml_source1_s20429 {
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
                                    let source_ml_source1_type_query_operand20569 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20556, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20556.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20557, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20557.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20558, _, _, _, _) => {
                                            source_ml_source1_data_property_value20558.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20559, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20559.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20560, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20560.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20561, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20561.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20562, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20562.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20563, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20563.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20564, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20564.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20565, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20565.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20566, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20566.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20567, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20567.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20568, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value20568.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20583, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20583.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20584, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20584.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20585, _, _, _, _) => {
                                        source_ml_source1_data_property_value20585.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20586, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20586.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20587, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20587.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20588, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20588.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20589, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20589.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20590, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20590.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20591, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20591.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20592, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20592.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20593, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20593.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20594, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20594.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20595, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20595.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20609, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20609.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20610, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20610.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20611, _, _, _, _) => {
                                        source_ml_source1_data_property_value20611.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20612, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20612.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20613, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20613.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20614, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20614.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20615, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20615.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20616, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20616.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20617, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20617.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20618, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20618.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20619, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20619.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20620, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20620.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20621, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value20621.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value20622, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20622.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value20623, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20623.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value20624, _, _, _, _) => {
                                                source_ml_source1_data_property_value20624.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value20625, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20625.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value20626, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20626.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value20627, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20627.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value20628, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20628.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value20629, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20629.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value20630, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20630.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value20631, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20631.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value20632, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20632.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value20633, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20633.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value20634, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value20634.clone()
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
pub fn source_ml_temporary_allocate_java_script_collection_iterator_loop135(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
    loop {
        if (at
            < ({
                let ml_reference_array184 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value21681,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21681.clone(),
                };
                ml_reference_array184.borrow().len() as f64
            }))
            && ((({
                let source_ml_source1_optional_chain_receiver21702 = ({
                    let source_ml_source1_array_read_value21700 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value21699,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value21699.clone(),
                    };
                    {
                        let ml_reference_array185 = source_ml_source1_array_read_value21700;
                        let ml_reference_position185 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array185.borrow(),
                            self::ml_array::number_index(ml_reference_position185),
                        )
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver21702.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_temporary_optional_chain133 =
                            ((source_ml_source1_optional_chain_receiver21702.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_temporary_optional_chain133 {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value21703,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21703.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value21704,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21704.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value21705,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21705.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value21706,
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
                            ) => source_ml_source1_data_property_value21706.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value21707,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21707.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value21708,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21708.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value21709,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21709.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value21710,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21710.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value21711,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21711.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value21712,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21712.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value21713,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21713.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value21714,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21714.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value21715,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21715.clone(),
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result {
    MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Done,
    MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Return(
        Box<self::JavaScriptCollectionIteration>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_collection_iterator_loop134(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values132: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index132: self::ml::Big,
) -> self::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result {
    loop {
        return if source_ml_temporary_index132
            < ({
                let ml_reference_array186 = source_ml_temporary_values132.clone();
                self::ml::Big::from_u128(ml_reference_array186.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array187 = source_ml_temporary_values132.clone();
                    let ml_reference_position187 = source_ml_temporary_index132.clone();
                    self::ml_array::at(
                        &*ml_reference_array187.borrow(),
                        ml_reference_position187.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index132_2 =
                                source_ml_temporary_index132.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index132 = source_ml_temporary_index132_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Return(Box::new(self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), invalid.clone())))
                    }
                }
            }
        } else {
            self::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result {
    MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Done,
    MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Return(
        Box<self::JavaScriptCollectionIteration>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_advance_java_script_collection_iterator_loop136(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut iterator: Option<self::JavaScriptOrdinaryObject>,
    mut collection_identity: self::ml::Big,
    kind: &self::ml_string::Text,
    mut entries: Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>,
    mut cursor: f64,
) -> self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result {
    loop {
        return if cursor
            < ({
                let source_ml_source1_nullish_field_receiver21720 = (entries.clone(),);
                if match (source_ml_source1_nullish_field_receiver21720.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present21721 =
                            ((source_ml_source1_nullish_field_receiver21720.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array188 =
                                source_ml_source1_nullish_field_present21721;
                            ml_reference_array188.borrow().len() as f64
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }) {
            {
                let entry = {
                    let source_ml_source1_nullish_field_receiver21722 = (entries.clone(), cursor);
                    {
                        let source_ml_source1_computed_property_key21723 =
                            (source_ml_source1_nullish_field_receiver21722.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver21722.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present21724 =
                                    ((source_ml_source1_nullish_field_receiver21722.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let ml_reference_array189 =
                                        source_ml_source1_nullish_field_present21724;
                                    let ml_reference_position189 =
                                        source_ml_source1_computed_property_key21723;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array189.borrow(),
                                        self::ml_array::number_index(ml_reference_position189),
                                    )
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (((self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39,
                                ]))
                                .concat(
                                    &self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        (source_ml_source1_nullish_field_receiver21722.clone()).1,
                                    )),
                                ))
                                .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                if match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || !(match entry.clone().expect("a proven present value") {
                    self::JavaScriptCollectionEntry::CollectionEntry(
                        _,
                        _,
                        source_ml_source1_data_property_value21727,
                    ) => source_ml_source1_data_property_value21727,
                }) {
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            cursor = cursor_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let updated = self::move_java_script_collection_iterator_cursor(
                            heap.clone(),
                            iterator.clone(),
                            collection_identity.clone(),
                            (cursor + 1f64),
                        );
                        self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Return(Box::new(self::JavaScriptCollectionIteration::CollectionIteration(Box::new(updated), false, Box::new(match entry.clone().expect("a proven present value") {
                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value21728, _, _) => {
                                let source_ml_source1_data_property_value21728 = *source_ml_source1_data_property_value21728;
                                source_ml_source1_data_property_value21728.clone()
                            }
                        }), Box::new(if (*kind) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])) {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value21729, _, _) => {
                                    let source_ml_source1_data_property_value21729 = *source_ml_source1_data_property_value21729;
                                    source_ml_source1_data_property_value21729.clone()
                                }
                            }
                        } else {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value21730, _) => {
                                    let source_ml_source1_data_property_value21730 = *source_ml_source1_data_property_value21730;
                                    source_ml_source1_data_property_value21730.clone()
                                }
                            }
                        }), ((*kind) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))))))
                    }
                }
            }
        } else {
            self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Done
        };
    }
}
