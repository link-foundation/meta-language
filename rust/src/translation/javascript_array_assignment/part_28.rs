// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
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
        return if source_ml_source1_temporary_index77
            < ({
                let ml_reference_array326 = source_ml_source1_temporary_values77.clone();
                self::ml::Big::from_u128(ml_reference_array326.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array327 = source_ml_source1_temporary_values77.clone();
                    let ml_reference_position327 = source_ml_source1_temporary_index77.clone();
                    self::ml_array::at(
                        &*ml_reference_array327.borrow(),
                        ml_reference_position327.to_index(),
                    )
                };
                {
                    let source_ml_source2_s19120 = match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, immutable_prototype, regular_expression, string_data, primitive_data, callable, _) => {
                            if {
                                let source_ml_source2_comparison_operand19211 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source2_comparison_operand19212 = None::<()>;
                                    !(match (source_ml_source2_comparison_operand19211, source_ml_source2_comparison_operand19212) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && ((match ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 66, 111, 111, 108, 101, 97, 110]))
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
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 66, 105, 103, 73, 110, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116]))
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
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 83, 116, 114, 105, 110, 103, 68, 97, 116, 97]))
                                            } else
                                                if {
                                                    let source_ml_source2_comparison_operand19231 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source2_comparison_operand19232 = None::<()>;
                                                        !(match (source_ml_source2_comparison_operand19231, source_ml_source2_comparison_operand19232) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                    }
                                                } && ((match ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Return(self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 115, 108, 111, 116]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                                                }

                                    }
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next
                        }
                    };
                    match source_ml_source2_s19120 {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin117Next => {
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
                                    let source_ml_source2_type_query_operand19260 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value19247, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19247.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value19248, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19248.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value19249, _, _, _, _) => {
                                            source_ml_source2_data_property_value19249.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value19250, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19250.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value19251, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19251.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value19252, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19252.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value19253, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19253.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value19254, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19254.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value19255, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19255.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value19256, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19256.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value19257, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19257.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value19258, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19258.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value19259, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19259.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value19274, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19274.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value19275, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19275.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value19276, _, _, _, _) => {
                                        source_ml_source2_data_property_value19276.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value19277, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19277.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value19278, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19278.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value19279, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19279.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value19280, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19280.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value19281, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19281.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value19282, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19282.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value19283, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19283.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value19284, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19284.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value19285, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19285.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value19286, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19286.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value19300, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19300.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value19301, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19301.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value19302, _, _, _, _) => {
                                        source_ml_source2_data_property_value19302.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value19303, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19303.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value19304, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19304.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value19305, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19305.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value19306, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19306.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value19307, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19307.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value19308, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19308.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value19309, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19309.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value19310, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19310.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value19311, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19311.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value19312, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19312.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value19313, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19313.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value19314, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19314.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value19315, _, _, _, _) => {
                                                source_ml_source2_data_property_value19315.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value19316, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19316.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value19317, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19317.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value19318, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19318.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value19319, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19319.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value19320, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19320.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value19321, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19321.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value19322, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19322.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value19323, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19323.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value19324, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19324.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value19325, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19325.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index77_2 = source_ml_source1_temporary_index77.add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_source1_temporary_index77) = (previous_2, source_ml_source1_temporary_index77_2);
                                                continue;
                                            }
                                        }
                                    }
                                }} else {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100]))
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
                std::rc::Rc<
                    std::cell::RefCell<(
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
                    )>,
                >,
                bool,
                self::ml::Big,
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
        return if depth
            < ({
                let ml_reference_array330 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value19327,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value19327.clone(),
                };
                ml_reference_array330.borrow().len() as f64
            }) {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source2_data_property_value19328,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value19328.clone(),
                    },
                    current.clone(),
                );
                {
                    let object = {
                        let source_ml_source2_array_read_value19330 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source2_data_property_value19329,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value19329.clone(),
                        };
                        {
                            let ml_reference_array331 = source_ml_source2_array_read_value19330;
                            let ml_reference_position331 = index;
                            self::ml_array::optional_at(
                                &*ml_reference_array331.borrow(),
                                self::ml_array::number_index(ml_reference_position331),
                            )
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
                            let at = self::find_java_script_ordinary_property(
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19332,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19332.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19333,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19333.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19334,
                                    ) => source_ml_source2_data_property_value19334.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19335,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19335.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19336,
                                        _,
                                    ) => source_ml_source2_data_property_value19336.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19337,
                                        _,
                                    ) => source_ml_source2_data_property_value19337.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19338,
                                        _,
                                    ) => source_ml_source2_data_property_value19338.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19339,
                                        _,
                                    ) => source_ml_source2_data_property_value19339.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19340,
                                    ) => source_ml_source2_data_property_value19340.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19341,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19341.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19342,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19342.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19343,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19343.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19344,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value19344.clone(),
                                },
                                key.clone(),
                            );
                            {
                                let property = {
                                    let source_ml_source2_array_read_value19358 = match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value19345, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19345.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value19346, _, _, _) => {
                                            source_ml_source2_data_property_value19346.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value19347) => {
                                            source_ml_source2_data_property_value19347.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value19348, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19348.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value19349, _) => {
                                            source_ml_source2_data_property_value19349.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value19350, _) => {
                                            source_ml_source2_data_property_value19350.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value19351, _) => {
                                            source_ml_source2_data_property_value19351.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value19352, _) => {
                                            source_ml_source2_data_property_value19352.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19353) => {
                                            source_ml_source2_data_property_value19353.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value19354, _, _) => {
                                            source_ml_source2_data_property_value19354.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value19355, _, _) => {
                                            source_ml_source2_data_property_value19355.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value19356, _, _) => {
                                            source_ml_source2_data_property_value19356.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value19357, _, _) => {
                                            source_ml_source2_data_property_value19357.clone()
                                        }
                                    };
                                    {
                                        let ml_reference_array332 =
                                            source_ml_source2_array_read_value19358;
                                        let ml_reference_position332 = at;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array332.borrow(),
                                            self::ml_array::number_index(ml_reference_position332),
                                        )
                                    }
                                };
                                if (match (property.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value19364, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19364 = *source_ml_source2_data_property_value19364;
                                                source_ml_source2_data_property_value19364.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value19365, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19365 = *source_ml_source2_data_property_value19365;
                                                source_ml_source2_data_property_value19365.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value19366, _, _, _) => {
                                                let source_ml_source2_data_property_value19366 = *source_ml_source2_data_property_value19366;
                                                source_ml_source2_data_property_value19366.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value19367, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19367 = *source_ml_source2_data_property_value19367;
                                                source_ml_source2_data_property_value19367.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value19368, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19368 = *source_ml_source2_data_property_value19368;
                                                source_ml_source2_data_property_value19368.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value19369, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19369 = *source_ml_source2_data_property_value19369;
                                                source_ml_source2_data_property_value19369.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value19370, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19370 = *source_ml_source2_data_property_value19370;
                                                source_ml_source2_data_property_value19370.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value19371, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19371 = *source_ml_source2_data_property_value19371;
                                                source_ml_source2_data_property_value19371.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value19372, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19372 = *source_ml_source2_data_property_value19372;
                                                source_ml_source2_data_property_value19372.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value19373, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19373 = *source_ml_source2_data_property_value19373;
                                                source_ml_source2_data_property_value19373.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value19374, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19374 = *source_ml_source2_data_property_value19374;
                                                source_ml_source2_data_property_value19374.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value19375, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19375 = *source_ml_source2_data_property_value19375;
                                                source_ml_source2_data_property_value19375.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value19376, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19376 = *source_ml_source2_data_property_value19376;
                                                source_ml_source2_data_property_value19376.clone()
                                            }
                                        };
                                        match prototype {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, _) => {
                                                {
                                                    let depth_2 = depth + 1f64;
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Return(std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field19363 = match property.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value19362) => {
                                                source_ml_source2_data_property_value19362.clone()
                                            }
                                        };
                                        (source_ml_source2_field19363, true, current.clone())
                                    })))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done,
    MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values136: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index136: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    loop {
        return if source_ml_source1_temporary_index136
            < ({
                let ml_reference_array333 = source_ml_source1_temporary_values136.clone();
                self::ml::Big::from_u128(ml_reference_array333.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array334 = source_ml_source1_temporary_values136.clone();
                    let ml_reference_position334 = source_ml_source1_temporary_index136.clone();
                    self::ml_array::at(
                        &*ml_reference_array334.borrow(),
                        ml_reference_position334.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_source1_temporary_index136_2 =
                                source_ml_source1_temporary_index136
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index136 =
                                    source_ml_source1_temporary_index136_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(invalid.clone())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop145(
    mut exponent: f64,
    mut value: self::ml::Big,
    mut base: self::ml::Big,
) -> self::ml::Big {
    loop {
        if exponent > 0f64 {
            {
                let value_3 = if (exponent % 2f64) == 1f64 {
                    { value.mul(&base) }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = (exponent / 2f64).floor();
                    if exponent_2 > 0f64 {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        }
        return { value.clone() };
    }
}
