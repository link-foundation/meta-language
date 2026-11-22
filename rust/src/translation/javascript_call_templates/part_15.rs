// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop87(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done
            }
            self::JavaScriptSemanticValue::Object(identity, _) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18259, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18259.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18260, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18260.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18261, _, _, _, _) => {
                            source_ml_source1_data_property_value18261.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18262, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18262.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18263, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18263.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18264, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18264.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18265, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18265.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18266, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18266.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18267, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18267.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18268, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18268.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18269, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18269.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18270, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18270.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18271, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18271.clone()
                        }
                    })) || (depth_2 > ({ let ml_reference_array143 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18273, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18273.clone()
                        }
                    }; ml_reference_array143.borrow().len() as f64 })) {
                        self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18274, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value18274.clone()
                                }
                            }, identity.clone());
                            {
                                let parent = {
                                    let source_ml_source1_array_read_value18276 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18275, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18275.clone()
                                        }
                                    };
                                    { let ml_reference_array144 = source_ml_source1_array_read_value18276; let ml_reference_position144 = index; self::ml_array::optional_at(&*ml_reference_array144.borrow(), self::ml_array::number_index(ml_reference_position144)) }
                                };
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
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
                                        self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 108, 111, 111, 107, 117, 112]))
                                    } else {
                                        {
                                            let prototype_2 = match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value19133, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19133 = *source_ml_source1_data_property_value19133;
                                                    source_ml_source1_data_property_value19133.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value19134, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19134 = *source_ml_source1_data_property_value19134;
                                                    source_ml_source1_data_property_value19134.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value19135, _, _, _) => {
                                                    let source_ml_source1_data_property_value19135 = *source_ml_source1_data_property_value19135;
                                                    source_ml_source1_data_property_value19135.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value19136, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19136 = *source_ml_source1_data_property_value19136;
                                                    source_ml_source1_data_property_value19136.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value19137, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19137 = *source_ml_source1_data_property_value19137;
                                                    source_ml_source1_data_property_value19137.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value19138, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19138 = *source_ml_source1_data_property_value19138;
                                                    source_ml_source1_data_property_value19138.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value19139, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19139 = *source_ml_source1_data_property_value19139;
                                                    source_ml_source1_data_property_value19139.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value19140, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19140 = *source_ml_source1_data_property_value19140;
                                                    source_ml_source1_data_property_value19140.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value19141, _, _, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19141 = *source_ml_source1_data_property_value19141;
                                                    source_ml_source1_data_property_value19141.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value19142, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19142 = *source_ml_source1_data_property_value19142;
                                                    source_ml_source1_data_property_value19142.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value19143, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19143 = *source_ml_source1_data_property_value19143;
                                                    source_ml_source1_data_property_value19143.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value19144, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19144 = *source_ml_source1_data_property_value19144;
                                                    source_ml_source1_data_property_value19144.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value19145, _, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value19145 = *source_ml_source1_data_property_value19145;
                                                    source_ml_source1_data_property_value19145.clone()
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
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop114(
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut source_ml_temporary_values103: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_temporary_index103: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Result {
    loop {
        if source_ml_temporary_index103
            < ({
                let ml_reference_array145 = source_ml_temporary_values103.clone();
                self::ml::Big::from_u128(ml_reference_array145.borrow().len() as u128)
            })
        {
            {
                let selected = {
                    let ml_reference_array146 = source_ml_temporary_values103.clone();
                    let ml_reference_position146 = source_ml_temporary_index103.clone();
                    self::ml_array::at(
                        &*ml_reference_array146.borrow(),
                        ml_reference_position146.to_index(),
                    )
                };
                {
                    let mark = {
                        let ml_reference_array147 = marks.clone();
                        let ml_reference_position147 = selected;
                        self::ml_array::optional_at(
                            &*ml_reference_array147.borrow(),
                            self::ml_array::number_index(ml_reference_position147),
                        )
                    };
                    if !(match (mark.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_temporary_read_java_script_ordinary_heap_error_written115 = {
                                let ml_field_owner148 =
                                    mark.clone().expect("a proven present value");
                                let ml_field_value148 = 2f64;
                                ml_field_owner148.borrow_mut().0 = ml_field_value148;
                                ()
                            };
                            {
                                let source_ml_temporary_index103_2 =
                                    source_ml_temporary_index103.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index103 = source_ml_temporary_index103_2;
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_temporary_index103_3 =
                                source_ml_temporary_index103.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index103 = source_ml_temporary_index103_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop113(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
    mut path: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result {
    loop {
        return if index >= 0f64 {
            {
                let current = {
                    let source_ml_source1_array_read_value19151 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value19150,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value19150.clone(),
                    };
                    {
                        let ml_reference_array149 = source_ml_source1_array_read_value19151;
                        let ml_reference_position149 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array149.borrow(),
                            self::ml_array::number_index(ml_reference_position149),
                        )
                    }
                };
                {
                    let mark = {
                        let ml_reference_array150 = marks.clone();
                        let ml_reference_position150 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array150.borrow(),
                            self::ml_array::number_index(ml_reference_position150),
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
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                    } else {
                        match current.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, _) => {
                                let bound_target = *bound_target;
                                if (({ (mark.clone().expect("a proven present value")).borrow().clone() }).0) == 1f64 {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 121, 99, 108, 105, 99, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let source_ml_temporary_read_java_script_ordinary_heap_error_written116 = { let ml_field_owner153 = mark.clone().expect("a proven present value"); let ml_field_value153 = 1f64; ml_field_owner153.borrow_mut().0 = ml_field_value153; () };
                                        {
                                            let source_ml_temporary_read_java_script_ordinary_heap_error_ignored117 = { let ml_method_owner154 = path.clone(); let ml_method_argument154_0 = index; let ml_inserted = vec![ml_method_argument154_0]; let mut ml_method_values154 = ml_method_owner154.borrow_mut(); let ml_length = ml_method_values154.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values154.extend(ml_inserted); ml_length as f64 };
                                            match bound_target.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                }
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let index_2 = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value19257, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value19257.clone()
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
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop112(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut first: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result {
    loop {
        return if first
            < ({
                let ml_reference_array155 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value19259,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value19259.clone(),
                };
                ml_reference_array155.borrow().len() as f64
            }) {
            {
                let index = first;
                {
                    let path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let source_ml_source1_s19260 =
                            self::source_ml_temporary_read_java_script_ordinary_heap_error_loop113(
                                heap.clone(),
                                marks.clone(),
                                index,
                                path.clone(),
                            );
                        match source_ml_source1_s19260 {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Done => {
                                {
                                    let source_ml_temporary_index103 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s19261 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop114(marks.clone(), path.clone(), source_ml_temporary_index103);
                                        match source_ml_source1_s19261 {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop114Done => {
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
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop113Return(source_ml_temporary_result115) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Return(source_ml_temporary_result115.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop111(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    loop {
        if index
            < ({
                let ml_reference_array156 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value19263,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value19263.clone(),
                };
                ml_reference_array156.borrow().len() as f64
            })
        {
            {
                let source_ml_temporary_read_java_script_ordinary_heap_error_ignored118 = {
                    let ml_method_owner157 = marks.clone();
                    let ml_method_argument157_0 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field19264 = 0f64;
                        (source_ml_source1_field19264,)
                    }));
                    let ml_inserted = vec![ml_method_argument157_0];
                    let mut ml_method_values157 = ml_method_owner157.borrow_mut();
                    let ml_length = ml_method_values157
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values157.extend(ml_inserted);
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Return(self::ml_string::Text),
}
