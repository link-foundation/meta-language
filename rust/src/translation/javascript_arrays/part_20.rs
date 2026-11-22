// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop76(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Object(identity, _) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18555, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18555.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18556, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18556.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18557, _, _, _, _) => {
                            source_ml_source1_data_property_value18557.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18558, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18558.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18559, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18559.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18560, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18560.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18561, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18561.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18562, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18562.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18563, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18563.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18564, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18564.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18565, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18565.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18566, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18566.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18567, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18567.clone()
                        }
                    })) || (depth_2 > ({ let ml_reference_array259 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18569, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18569.clone()
                        }
                    }; ml_reference_array259.borrow().len() as f64 })) {
                        self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18570, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value18570.clone()
                                }
                            }, identity.clone());
                            {
                                let parent = {
                                    let source_ml_source1_array_read_value18572 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18571, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18571.clone()
                                        }
                                    };
                                    { let ml_reference_array260 = source_ml_source1_array_read_value18572; let ml_reference_position260 = index; self::ml_array::optional_at(&*ml_reference_array260.borrow(), self::ml_array::number_index(ml_reference_position260)) }
                                };
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else if (((((((match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    } && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match parent.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    } {
                                        self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 108, 111, 111, 107, 117, 112]))
                                    } else {
                                        {
                                            let prototype_2 = match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value19429, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19429 = *source_ml_source1_data_property_value19429;
                                                    source_ml_source1_data_property_value19429.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value19430, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19430 = *source_ml_source1_data_property_value19430;
                                                    source_ml_source1_data_property_value19430.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value19431, _, _, _) => {
                                                    let source_ml_source1_data_property_value19431 = *source_ml_source1_data_property_value19431;
                                                    source_ml_source1_data_property_value19431.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value19432, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19432 = *source_ml_source1_data_property_value19432;
                                                    source_ml_source1_data_property_value19432.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value19433, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19433 = *source_ml_source1_data_property_value19433;
                                                    source_ml_source1_data_property_value19433.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value19434, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19434 = *source_ml_source1_data_property_value19434;
                                                    source_ml_source1_data_property_value19434.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value19435, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19435 = *source_ml_source1_data_property_value19435;
                                                    source_ml_source1_data_property_value19435.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value19436, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19436 = *source_ml_source1_data_property_value19436;
                                                    source_ml_source1_data_property_value19436.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value19437, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19437 = *source_ml_source1_data_property_value19437;
                                                    source_ml_source1_data_property_value19437.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value19438, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19438 = *source_ml_source1_data_property_value19438;
                                                    source_ml_source1_data_property_value19438.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value19439, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19439 = *source_ml_source1_data_property_value19439;
                                                    source_ml_source1_data_property_value19439.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value19440, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19440 = *source_ml_source1_data_property_value19440;
                                                    source_ml_source1_data_property_value19440.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value19441, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19441 = *source_ml_source1_data_property_value19441;
                                                    source_ml_source1_data_property_value19441.clone()
                                                }
                                            };
                                            {
                                                (prototype, depth) = (prototype_2, depth_2);
                                                continue;
                                            }
                                        }
                                    }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop103(
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut source_ml_temporary_values92: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_temporary_index92: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    loop {
        if source_ml_temporary_index92
            < ({
                let ml_reference_array261 = source_ml_temporary_values92.clone();
                self::ml::Big::from_u128(ml_reference_array261.borrow().len() as u128)
            })
        {
            {
                let selected = {
                    let ml_reference_array262 = source_ml_temporary_values92.clone();
                    let ml_reference_position262 = source_ml_temporary_index92.clone();
                    self::ml_array::at(
                        &*ml_reference_array262.borrow(),
                        ml_reference_position262.to_index(),
                    )
                };
                {
                    let mark = {
                        let ml_reference_array263 = marks.clone();
                        let ml_reference_position263 = selected;
                        self::ml_array::optional_at(
                            &*ml_reference_array263.borrow(),
                            self::ml_array::number_index(ml_reference_position263),
                        )
                    };
                    if !(match (mark.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_temporary_read_java_script_ordinary_heap_error_written104 = {
                                let ml_field_owner264 =
                                    mark.clone().expect("a proven present value");
                                let ml_field_value264 = 2f64;
                                ml_field_owner264.borrow_mut().0 = ml_field_value264;
                                ()
                            };
                            {
                                let source_ml_temporary_index92_2 =
                                    source_ml_temporary_index92.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index92 = source_ml_temporary_index92_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index92_3 =
                                source_ml_temporary_index92.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index92 = source_ml_temporary_index92_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop102(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
    mut path: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    loop {
        return if index >= 0f64 {
            {
                let current = {
                    let source_ml_source1_array_read_value19447 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value19446,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19446.clone(),
                    };
                    {
                        let ml_reference_array265 = source_ml_source1_array_read_value19447;
                        let ml_reference_position265 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array265.borrow(),
                            self::ml_array::number_index(ml_reference_position265),
                        )
                    }
                };
                {
                    let mark = {
                        let ml_reference_array266 = marks.clone();
                        let ml_reference_position266 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array266.borrow(),
                            self::ml_array::number_index(ml_reference_position266),
                        )
                    };
                    if (match (current.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match (mark.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) || ((({
                        (mark.clone().expect("a proven present value"))
                            .borrow()
                            .clone()
                    })
                    .0) == 2f64)
                    {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                    } else {
                        match current.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, _) => {
                                let bound_target = *bound_target;
                                if (({ (mark.clone().expect("a proven present value")).borrow().clone() }).0) == 1f64 {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 121, 99, 108, 105, 99, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let source_ml_temporary_read_java_script_ordinary_heap_error_written105 = { let ml_field_owner269 = mark.clone().expect("a proven present value"); let ml_field_value269 = 1f64; ml_field_owner269.borrow_mut().0 = ml_field_value269; () };
                                        {
                                            let source_ml_temporary_read_java_script_ordinary_heap_error_ignored106 = { let ml_method_owner270 = path.clone(); let ml_method_argument270_0 = index; let ml_inserted = vec![ml_method_argument270_0]; let mut ml_method_values270 = ml_method_owner270.borrow_mut(); let ml_length = ml_method_values270.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values270.extend(ml_inserted); ml_length as f64 };
                                            match bound_target.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let index_2 = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value19553, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value19553.clone()
                                                            }
                                                        }, identity.clone());
                                                        {
                                                            index = index_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop101(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut first: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    loop {
        return if first
            < ({
                let ml_reference_array271 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value19555,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value19555.clone(),
                };
                ml_reference_array271.borrow().len() as f64
            }) {
            {
                let index = first;
                {
                    let path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let source_ml_source1_s19556 =
                            self::source_ml_temporary_read_java_script_ordinary_heap_error_loop102(
                                heap.clone(),
                                marks.clone(),
                                index,
                                path.clone(),
                            );
                        match source_ml_source1_s19556 {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done => {
                                {
                                    let source_ml_temporary_index92 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s19557 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop103(marks.clone(), path.clone(), source_ml_temporary_index92);
                                        match source_ml_source1_s19557 {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done => {
                                                {
                                                    let first_2 = first + 1f64;
                                                    {
                                                        first = first_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(source_ml_temporary_result104) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(source_ml_temporary_result104.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop100(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result {
    loop {
        if index
            < ({
                let ml_reference_array272 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value19559,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value19559.clone(),
                };
                ml_reference_array272.borrow().len() as f64
            })
        {
            {
                let source_ml_temporary_read_java_script_ordinary_heap_error_ignored107 = {
                    let ml_method_owner273 = marks.clone();
                    let ml_method_argument273_0 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field19560 = 0f64;
                        (source_ml_source1_field19560,)
                    }));
                    let ml_inserted = vec![ml_method_argument273_0];
                    let mut ml_method_values273 = ml_method_owner273.borrow_mut();
                    let ml_length = ml_method_values273
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values273.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(self::ml_string::Text),
}
