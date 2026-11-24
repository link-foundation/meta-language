// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result;
use super::compare_java_script_collection_keys;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_object_error;
use super::read_java_script_bound_object_error;
use super::read_java_script_closure_object_error;
use super::read_java_script_collection_iterator_object_error;
use super::read_java_script_opaque_object_error;
use super::read_java_script_ordinary_object_descriptor_error;
use super::read_java_script_ordinary_prototype_chain_error;
use super::read_java_script_strong_collection_object_error;
use super::read_java_script_weak_collection_object_error;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<
        std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<((f64,), Vec<f64>)>>>>,
    >,
    mut index: f64,
    mut path: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
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
                if (index >= (0f64)) {
                    {
                        let current = {
                            let source_ml_source2_array_read_value39335 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2583 =
                                    source_ml_source2_array_read_value39335.clone();
                                let ml_reference_position2583 = index;
                                let ml_reference_projection2583 = self::ml_array::optional_at(
                                    &*ml_reference_array2583.borrow(),
                                    self::ml_array::number_index(ml_reference_position2583),
                                );
                                ml_reference_projection2583
                            }
                        };
                        {
                            let mark = {
                                let ml_reference_array2584 = marks.clone();
                                let ml_reference_position2584 = index;
                                let ml_reference_projection2584 = self::ml_array::optional_at(
                                    &*ml_reference_array2584.borrow(),
                                    self::ml_array::number_index(ml_reference_position2584),
                                );
                                ml_reference_projection2584
                            };
                            if ((match (current.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match (mark.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) || (((({
                                let ml_reference_value2585 =
                                    (mark.clone().expect("a proven present value"))
                                        .borrow()
                                        .clone();
                                ml_reference_value2585
                            })
                            .0)
                                .0)
                                == (2f64)))
                            {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                            } else {
                                match current.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, _) => {
                                        let bound_target = *bound_target;
                                        if (((({ let ml_reference_value2586 = (mark.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2586 }).0).0) == (1f64)) {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 121, 99, 108, 105, 99, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114]))
                                        } else {
                                            {
                                                let source_ml_source1_temporary_read_java_script_ordinary_heap_error_written105 = { let ml_field_owner2587 = mark.clone().expect("a proven present value"); let ml_field_value2587 = 1f64; ml_field_owner2587.borrow_mut().0.0 = ml_field_value2587; () };
                                                {
                                                    let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored106 = { let ml_method_owner2588 = path.clone(); let ml_method_argument2588_0 = index; let ml_inserted = vec![ml_method_argument2588_0]; let mut ml_method_values2588 = ml_method_owner2588.borrow_mut(); let ml_length = ml_method_values2588.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2588.extend(ml_inserted); ml_length as f64 };
                                                    match bound_target.clone() {
                                                        self::JavaScriptSemanticValue::Object(identity, _) => {
                                                            {
                                                                let index_2 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                {
                                                                    index = index_2;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop101(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut marks: std::rc::Rc<
        std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<((f64,), Vec<f64>)>>>>,
    >,
    mut first: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    loop {
        return if (first
            < ({
                let ml_reference_array2589 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value39349,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39349.clone(),
                };
                let ml_reference_projection2589 = ml_reference_array2589.borrow().len() as f64;
                ml_reference_projection2589
            })) {
            {
                let index = first;
                {
                    let path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let source_ml_source2_s39350 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(heap.clone(), marks.clone(), index, path.clone());
                        match source_ml_source2_s39350.clone() {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done => {
                                {
                                    let source_ml_source1_temporary_index92 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s39351 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(marks.clone(), path.clone(), source_ml_source1_temporary_index92.clone());
                                        match source_ml_source2_s39351.clone() {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done => {
                                                {
                                                    let first_2 = (first + 1f64);
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
    mut marks: std::rc::Rc<
        std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<((f64,), Vec<f64>)>>>>,
    >,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result {
    loop {
        return if (index
            < ({
                let ml_reference_array2590 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value39353,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39353.clone(),
                };
                let ml_reference_projection2590 = ml_reference_array2590.borrow().len() as f64;
                ml_reference_projection2590
            })) {
            {
                let source_ml_source1_temporary_read_java_script_ordinary_heap_error_ignored107 = {
                    let ml_method_owner2591 = marks.clone();
                    let ml_method_argument2591_0 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field39354 = 0f64;
                        {
                            let source_ml_source3_record_field493 = source_ml_source2_field39354;
                            ((source_ml_source3_record_field493,), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![115f64, 116f64, 97f64, 116f64, 101f64]], vec![0f64]))
                        }
                    }));
                    let ml_inserted = vec![ml_method_argument2591_0];
                    let mut ml_method_values2591 = ml_method_owner2591.borrow_mut();
                    let ml_length = ml_method_values2591
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2591.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop99(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
    mut source_ml_source1_temporary_values91: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index91: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result {
    loop {
        return if (source_ml_source1_temporary_index91
            < ({
                let ml_reference_array2592 = source_ml_source1_temporary_values91.clone();
                let ml_reference_projection2592 =
                    self::ml::Big::from_u128(ml_reference_array2592.borrow().len() as u128);
                ml_reference_projection2592
            })) {
            {
                let object = {
                    let ml_reference_array2593 = source_ml_source1_temporary_values91.clone();
                    let ml_reference_position2593 = source_ml_source1_temporary_index91.clone();
                    let ml_reference_projection2593 = self::ml_array::at(
                        &*ml_reference_array2593.borrow(),
                        ml_reference_position2593.to_index(),
                    );
                    ml_reference_projection2593
                };
                {
                    let opaque_object_error =
                        self::read_java_script_opaque_object_error(heap.clone(), object.clone());
                    if (opaque_object_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(opaque_object_error.clone())
                    } else {
                        {
                            let closure_object_error = self::read_java_script_closure_object_error(
                                heap.clone(),
                                object.clone(),
                                budget.clone(),
                            );
                            if (closure_object_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(closure_object_error.clone())
                            } else {
                                {
                                    let bound_object_error =
                                        self::read_java_script_bound_object_error(
                                            heap.clone(),
                                            object.clone(),
                                            budget.clone(),
                                        );
                                    if (bound_object_error
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(bound_object_error.clone())
                                    } else {
                                        {
                                            let collection_iterator_object_error = self::read_java_script_collection_iterator_object_error(heap.clone(), object.clone(), budget.clone());
                                            if (collection_iterator_object_error
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(collection_iterator_object_error.clone())
                                            } else {
                                                {
                                                    let weak_collection_object_error = self::read_java_script_weak_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                    if (weak_collection_object_error
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(weak_collection_object_error.clone())
                                                    } else {
                                                        {
                                                            let strong_collection_object_error = self::read_java_script_strong_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                            if (strong_collection_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(strong_collection_object_error.clone())
                                                            } else {
                                                                {
                                                                    let ordinary_object_descriptor_error = self::read_java_script_ordinary_object_descriptor_error(heap.clone(), match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value39357, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39357 = *source_ml_source2_data_property_value39357;
                                                                            source_ml_source2_data_property_value39357.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value39358, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39358 = *source_ml_source2_data_property_value39358;
                                                                            source_ml_source2_data_property_value39358.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value39359, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39359 = *source_ml_source2_data_property_value39359;
                                                                            source_ml_source2_data_property_value39359.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value39360, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39360 = *source_ml_source2_data_property_value39360;
                                                                            source_ml_source2_data_property_value39360.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value39361, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39361 = *source_ml_source2_data_property_value39361;
                                                                            source_ml_source2_data_property_value39361.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value39362, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39362 = *source_ml_source2_data_property_value39362;
                                                                            source_ml_source2_data_property_value39362.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value39363, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39363 = *source_ml_source2_data_property_value39363;
                                                                            source_ml_source2_data_property_value39363.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value39364, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39364 = *source_ml_source2_data_property_value39364;
                                                                            source_ml_source2_data_property_value39364.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value39365, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39365 = *source_ml_source2_data_property_value39365;
                                                                            source_ml_source2_data_property_value39365.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value39366, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39366 = *source_ml_source2_data_property_value39366;
                                                                            source_ml_source2_data_property_value39366.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value39367, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39367 = *source_ml_source2_data_property_value39367;
                                                                            source_ml_source2_data_property_value39367.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value39368, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39368 = *source_ml_source2_data_property_value39368;
                                                                            source_ml_source2_data_property_value39368.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value39369, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value39369 = *source_ml_source2_data_property_value39369;
                                                                            source_ml_source2_data_property_value39369.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value39370, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value39370.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value39371, _, _, _) => {
                                                                            source_ml_source2_data_property_value39371.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value39372) => {
                                                                            source_ml_source2_data_property_value39372.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value39373, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value39373.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value39374, _) => {
                                                                            source_ml_source2_data_property_value39374.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value39375, _) => {
                                                                            source_ml_source2_data_property_value39375.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value39376, _) => {
                                                                            source_ml_source2_data_property_value39376.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value39377, _) => {
                                                                            source_ml_source2_data_property_value39377.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value39378) => {
                                                                            source_ml_source2_data_property_value39378.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value39379, _, _) => {
                                                                            source_ml_source2_data_property_value39379.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value39380, _, _) => {
                                                                            source_ml_source2_data_property_value39380.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value39381, _, _) => {
                                                                            source_ml_source2_data_property_value39381.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value39382, _, _) => {
                                                                            source_ml_source2_data_property_value39382.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    if (ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s39388 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                    {
                                                                                        let array_error = self::read_java_script_array_object_error(object.clone());
                                                                                        if (array_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(array_error.clone())
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s39388.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next => {
                                                                                    {
                                                                                        let ordinary_prototype_chain_error = self::read_java_script_ordinary_prototype_chain_error(heap.clone(), object.clone());
                                                                                        if (ordinary_prototype_chain_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_prototype_chain_error.clone())
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source1_temporary_index91_2 = source_ml_source1_temporary_index91.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    source_ml_source1_temporary_index91 = source_ml_source1_temporary_index91_2.clone();
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(source_ml_source1_temporary_result108) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(source_ml_source1_temporary_result108.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop112(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut key: self::JavaScriptSemanticValue,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array2594 = registry.clone();
                        let ml_reference_position2594 = middle;
                        let ml_reference_projection2594 = self::ml_array::optional_at(
                            &*ml_reference_array2594.borrow(),
                            self::ml_array::number_index(ml_reference_position2594),
                        );
                        ml_reference_projection2594
                    };
                    if (!(match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && ((self::compare_java_script_collection_keys(
                        match entry.clone().expect("a proven present value") {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source2_data_property_value39397,
                            ) => {
                                let source_ml_source2_data_property_value39397 =
                                    *source_ml_source2_data_property_value39397;
                                source_ml_source2_data_property_value39397.clone()
                            }
                        },
                        key.clone(),
                    )) < (0f64)))
                    {
                        {
                            let low_2 = (middle + 1f64);
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop111(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source_ml_source1_temporary_values89: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_source1_temporary_index89: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    loop {
        return if (source_ml_source1_temporary_index89
            < ({
                let ml_reference_array2595 = source_ml_source1_temporary_values89.clone();
                let ml_reference_projection2595 =
                    self::ml::Big::from_u128(ml_reference_array2595.borrow().len() as u128);
                ml_reference_projection2595
            })) {
            {
                let fact = {
                    let ml_reference_array2596 = source_ml_source1_temporary_values89.clone();
                    let ml_reference_position2596 = source_ml_source1_temporary_index89.clone();
                    let ml_reference_projection2596 = self::ml_array::at(
                        &*ml_reference_array2596.borrow(),
                        ml_reference_position2596.to_index(),
                    );
                    ml_reference_projection2596
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source2_data_property_value39400,
                        ) => ((source_ml_source2_data_property_value39400
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    if !(match (key.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let low = 0f64;
                            {
                                let high = {
                                    let ml_reference_array2597 = registry.clone();
                                    let ml_reference_projection2597 =
                                        ml_reference_array2597.borrow().len() as f64;
                                    ml_reference_projection2597
                                };
                                {
                                    let low_2 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop112(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source2_comparison_operand39406 = {
                                            let source_ml_source2_optional_chain_receiver39403 =
                                                ({
                                                    let ml_reference_array2598 = registry.clone();
                                                    let ml_reference_position2598 = low_2;
                                                    let ml_reference_projection2598 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array2598.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position2598,
                                                            ),
                                                        );
                                                    ml_reference_projection2598
                                                },);
                                            if match (source_ml_source2_optional_chain_receiver39403.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain90 = ((source_ml_source2_optional_chain_receiver39403.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain90.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value39404, _) => {
                                                            source_ml_source2_data_property_value39404.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source2_comparison_operand39407 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value39405, _, _) => {
                                                    source_ml_source2_data_property_value39405.clone()
                                                }
                                            };
                                            !(match (
                                                source_ml_source2_comparison_operand39406.clone(),
                                                source_ml_source2_comparison_operand39407.clone(),
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            })
                                        }
                                    } {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(self::ml_string::Text::from_units(vec![107, 110, 111, 119, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 109, 111, 110, 111, 116, 111, 110, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index89_2 =
                                                source_ml_source1_temporary_index89
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index89 =
                                                    source_ml_source1_temporary_index89_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index89_3 =
                                source_ml_source1_temporary_index89
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index89 =
                                    source_ml_source1_temporary_index89_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop110(
    mut previous_identity: Option<self::JavaScriptSemanticValue>,
    mut source_ml_source1_temporary_values88: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index88: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    loop {
        return if (source_ml_source1_temporary_index88
            < ({
                let ml_reference_array2599 = source_ml_source1_temporary_values88.clone();
                let ml_reference_projection2599 =
                    self::ml::Big::from_u128(ml_reference_array2599.borrow().len() as u128);
                ml_reference_projection2599
            })) {
            {
                let current = {
                    let ml_reference_array2600 = source_ml_source1_temporary_values88.clone();
                    let ml_reference_position2600 = source_ml_source1_temporary_index88.clone();
                    let ml_reference_projection2600 = self::ml_array::at(
                        &*ml_reference_array2600.borrow(),
                        ml_reference_position2600.to_index(),
                    );
                    ml_reference_projection2600
                };
                if (!(match (previous_identity.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    previous_identity.clone().expect("a proven present value"),
                    current.clone(),
                )) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121]))
                } else {
                    {
                        let source_ml_source1_temporary_index88_2 =
                            source_ml_source1_temporary_index88.add(&self::ml::Big::from_i128(1));
                        {
                            (previous_identity, source_ml_source1_temporary_index88) = (
                                Some(current.clone()),
                                source_ml_source1_temporary_index88_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text),
}
