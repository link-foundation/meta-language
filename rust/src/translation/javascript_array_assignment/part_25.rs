// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_prototype_chain_error_loop76(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
            }
            self::JavaScriptSemanticValue::Object(identity, _) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value17658, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17658.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value17659, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17659.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value17660, _, _, _, _) => {
                            source_ml_source2_data_property_value17660.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value17661, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17661.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value17662, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17662.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value17663, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17663.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value17664, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17664.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value17665, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17665.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value17666, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17666.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value17667, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17667.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value17668, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17668.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value17669, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17669.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value17670, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17670.clone()
                        }
                    })) || (depth_2 > ({ let ml_reference_array287 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17672, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17672.clone()
                        }
                    }; ml_reference_array287.borrow().len() as f64 })) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17673, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17673.clone()
                                }
                            }, identity.clone());
                            {
                                let parent = {
                                    let source_ml_source2_array_read_value17675 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17674, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17674.clone()
                                        }
                                    };
                                    { let ml_reference_array288 = source_ml_source2_array_read_value17675; let ml_reference_position288 = index; self::ml_array::optional_at(&*ml_reference_array288.borrow(), self::ml_array::number_index(ml_reference_position288)) }
                                };
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
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
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 108, 111, 111, 107, 117, 112]))
                                    } else {
                                        {
                                            let prototype_2 = match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value18532, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18532 = *source_ml_source2_data_property_value18532;
                                                    source_ml_source2_data_property_value18532.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value18533, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18533 = *source_ml_source2_data_property_value18533;
                                                    source_ml_source2_data_property_value18533.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value18534, _, _, _) => {
                                                    let source_ml_source2_data_property_value18534 = *source_ml_source2_data_property_value18534;
                                                    source_ml_source2_data_property_value18534.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value18535, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18535 = *source_ml_source2_data_property_value18535;
                                                    source_ml_source2_data_property_value18535.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value18536, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18536 = *source_ml_source2_data_property_value18536;
                                                    source_ml_source2_data_property_value18536.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value18537, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18537 = *source_ml_source2_data_property_value18537;
                                                    source_ml_source2_data_property_value18537.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value18538, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18538 = *source_ml_source2_data_property_value18538;
                                                    source_ml_source2_data_property_value18538.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value18539, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18539 = *source_ml_source2_data_property_value18539;
                                                    source_ml_source2_data_property_value18539.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value18540, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18540 = *source_ml_source2_data_property_value18540;
                                                    source_ml_source2_data_property_value18540.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value18541, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18541 = *source_ml_source2_data_property_value18541;
                                                    source_ml_source2_data_property_value18541.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value18542, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18542 = *source_ml_source2_data_property_value18542;
                                                    source_ml_source2_data_property_value18542.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value18543, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18543 = *source_ml_source2_data_property_value18543;
                                                    source_ml_source2_data_property_value18543.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value18544, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18544 = *source_ml_source2_data_property_value18544;
                                                    source_ml_source2_data_property_value18544.clone()
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
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut source_ml_source1_temporary_values92: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index92: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    loop {
        if source_ml_source1_temporary_index92
            < ({
                let ml_reference_array289 = source_ml_source1_temporary_values92.clone();
                self::ml::Big::from_u128(ml_reference_array289.borrow().len() as u128)
            })
        {
            {
                let selected = {
                    let ml_reference_array290 = source_ml_source1_temporary_values92.clone();
                    let ml_reference_position290 = source_ml_source1_temporary_index92.clone();
                    self::ml_array::at(
                        &*ml_reference_array290.borrow(),
                        ml_reference_position290.to_index(),
                    )
                };
                {
                    let mark = {
                        let ml_reference_array291 = marks.clone();
                        let ml_reference_position291 = selected;
                        self::ml_array::optional_at(
                            &*ml_reference_array291.borrow(),
                            self::ml_array::number_index(ml_reference_position291),
                        )
                    };
                    if !(match (mark.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_source1_temporary_read_java_script_ordinary_heap_error_written104 = {
                                let ml_field_owner292 =
                                    mark.clone().expect("a proven present value");
                                let ml_field_value292 = 2f64;
                                ml_field_owner292.borrow_mut().0 = ml_field_value292;
                                ()
                            };
                            {
                                let source_ml_source1_temporary_index92_2 =
                                    source_ml_source1_temporary_index92
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index92 =
                                        source_ml_source1_temporary_index92_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index92_3 =
                                source_ml_source1_temporary_index92
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index92 =
                                    source_ml_source1_temporary_index92_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
    mut path: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    loop {
        return if index >= 0f64 {
            {
                let current = {
                    let source_ml_source2_array_read_value18550 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source2_data_property_value18549,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value18549.clone(),
                    };
                    {
                        let ml_reference_array293 = source_ml_source2_array_read_value18550;
                        let ml_reference_position293 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array293.borrow(),
                            self::ml_array::number_index(ml_reference_position293),
                        )
                    }
                };
                {
                    let mark = {
                        let ml_reference_array294 = marks.clone();
                        let ml_reference_position294 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array294.borrow(),
                            self::ml_array::number_index(ml_reference_position294),
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
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                    } else {
                        match current.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, _) => {
                                let bound_target = *bound_target;
                                if (({ (mark.clone().expect("a proven present value")).borrow().clone() }).0) == 1f64 {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 121, 99, 108, 105, 99, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_read_java_script_ordinary_heap_error_written105 = { let ml_field_owner297 = mark.clone().expect("a proven present value"); let ml_field_value297 = 1f64; ml_field_owner297.borrow_mut().0 = ml_field_value297; () };
                                        {
                                            let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored106 = { let ml_method_owner298 = path.clone(); let ml_method_argument298_0 = index; let ml_inserted = vec![ml_method_argument298_0]; let mut ml_method_values298 = ml_method_owner298.borrow_mut(); let ml_length = ml_method_values298.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values298.extend(ml_inserted); ml_length as f64 };
                                            match bound_target.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let index_2 = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value18656, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18656.clone()
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
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop101(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut first: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    loop {
        return if first
            < ({
                let ml_reference_array299 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value18658,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value18658.clone(),
                };
                ml_reference_array299.borrow().len() as f64
            }) {
            {
                let index = first;
                {
                    let path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let source_ml_source2_s18659 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(heap.clone(), marks.clone(), index, path.clone());
                        match source_ml_source2_s18659 {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done => {
                                {
                                    let source_ml_source1_temporary_index92 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s18660 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(marks.clone(), path.clone(), source_ml_source1_temporary_index92);
                                        match source_ml_source2_s18660 {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done => {
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
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(source_ml_source1_temporary_result104) => {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(source_ml_source1_temporary_result104.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop100(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result {
    loop {
        if index
            < ({
                let ml_reference_array300 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value18662,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value18662.clone(),
                };
                ml_reference_array300.borrow().len() as f64
            })
        {
            {
                let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored107 = {
                    let ml_method_owner301 = marks.clone();
                    let ml_method_argument301_0 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field18663 = 0f64;
                        (source_ml_source2_field18663,)
                    }));
                    let ml_inserted = vec![ml_method_argument301_0];
                    let mut ml_method_values301 = ml_method_owner301.borrow_mut();
                    let ml_length = ml_method_values301
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values301.extend(ml_inserted);
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(self::ml_string::Text),
}
