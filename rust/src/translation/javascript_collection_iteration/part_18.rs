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
                    let source_ml_source1_s19119 = match object.clone() {
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
                                let source_ml_source1_comparison_operand19210 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand19211 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand19210, source_ml_source1_comparison_operand19211) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                                                    let source_ml_source1_comparison_operand19230 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand19231 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand19230, source_ml_source1_comparison_operand19231) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                    match source_ml_source1_s19119 {
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
                                    let source_ml_source1_type_query_operand19259 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value19246, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19246.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value19247, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19247.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value19248, _, _, _, _) => {
                                            source_ml_source1_data_property_value19248.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value19249, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19249.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value19250, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19250.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value19251, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19251.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value19252, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19252.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value19253, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19253.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value19254, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19254.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value19255, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19255.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value19256, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19256.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value19257, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19257.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value19258, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value19258.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value19273, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19273.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value19274, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19274.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value19275, _, _, _, _) => {
                                        source_ml_source1_data_property_value19275.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value19276, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19276.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value19277, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19277.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value19278, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19278.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value19279, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19279.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value19280, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19280.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value19281, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19281.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value19282, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19282.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value19283, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19283.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value19284, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19284.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value19285, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19285.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value19299, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19299.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value19300, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19300.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value19301, _, _, _, _) => {
                                        source_ml_source1_data_property_value19301.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value19302, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19302.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value19303, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19303.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value19304, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19304.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value19305, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19305.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value19306, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19306.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value19307, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19307.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value19308, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19308.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value19309, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19309.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value19310, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19310.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value19311, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value19311.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value19312, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19312.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value19313, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19313.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value19314, _, _, _, _) => {
                                                source_ml_source1_data_property_value19314.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value19315, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19315.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value19316, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19316.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value19317, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19317.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value19318, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19318.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value19319, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19319.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value19320, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19320.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value19321, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19321.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value19322, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19322.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value19323, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19323.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value19324, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value19324.clone()
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
                        source_ml_source1_data_property_value20371,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value20371.clone(),
                };
                ml_reference_array184.borrow().len() as f64
            }))
            && ((({
                let source_ml_source1_optional_chain_receiver20392 = ({
                    let source_ml_source1_array_read_value20390 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value20389,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value20389.clone(),
                    };
                    {
                        let ml_reference_array185 = source_ml_source1_array_read_value20390;
                        let ml_reference_position185 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array185.borrow(),
                            self::ml_array::number_index(ml_reference_position185),
                        )
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver20392.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_temporary_optional_chain133 =
                            ((source_ml_source1_optional_chain_receiver20392.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_temporary_optional_chain133 {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value20393,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20393.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value20394,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20394.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value20395,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20395.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value20396,
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
                            ) => source_ml_source1_data_property_value20396.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value20397,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20397.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value20398,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20398.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value20399,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20399.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value20400,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20400.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value20401,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20401.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value20402,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20402.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value20403,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20403.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value20404,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20404.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value20405,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value20405.clone(),
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
                let source_ml_source1_nullish_field_receiver20410 = (entries.clone(),);
                if match (source_ml_source1_nullish_field_receiver20410.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present20411 =
                            ((source_ml_source1_nullish_field_receiver20410.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array188 =
                                source_ml_source1_nullish_field_present20411;
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
                    let source_ml_source1_nullish_field_receiver20412 = (entries.clone(), cursor);
                    {
                        let source_ml_source1_computed_property_key20413 =
                            (source_ml_source1_nullish_field_receiver20412.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver20412.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present20414 =
                                    ((source_ml_source1_nullish_field_receiver20412.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let ml_reference_array189 =
                                        source_ml_source1_nullish_field_present20414;
                                    let ml_reference_position189 =
                                        source_ml_source1_computed_property_key20413;
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
                                        (source_ml_source1_nullish_field_receiver20412.clone()).1,
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
                        source_ml_source1_data_property_value20417,
                    ) => source_ml_source1_data_property_value20417,
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
                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value20418, _, _) => {
                                let source_ml_source1_data_property_value20418 = *source_ml_source1_data_property_value20418;
                                source_ml_source1_data_property_value20418.clone()
                            }
                        }), Box::new(if (*kind) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])) {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value20419, _, _) => {
                                    let source_ml_source1_data_property_value20419 = *source_ml_source1_data_property_value20419;
                                    source_ml_source1_data_property_value20419.clone()
                                }
                            }
                        } else {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value20420, _) => {
                                    let source_ml_source1_data_property_value20420 = *source_ml_source1_data_property_value20420;
                                    source_ml_source1_data_property_value20420.clone()
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
