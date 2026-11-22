// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result;
use super::compare_java_script_collection_keys;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
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
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop99(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut source_ml_temporary_values91: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index91: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result {
    loop {
        return if source_ml_temporary_index91
            < ({
                let ml_reference_array274 = source_ml_temporary_values91.clone();
                self::ml::Big::from_u128(ml_reference_array274.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array275 = source_ml_temporary_values91.clone();
                    let ml_reference_position275 = source_ml_temporary_index91.clone();
                    self::ml_array::at(
                        &*ml_reference_array275.borrow(),
                        ml_reference_position275.to_index(),
                    )
                };
                {
                    let opaque_object_error =
                        self::read_java_script_opaque_object_error(heap.clone(), object.clone());
                    {
                        let ml_library_inequality_9 =
                            opaque_object_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_9 {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(opaque_object_error.clone())
                        } else {
                            {
                                let closure_object_error =
                                    self::read_java_script_closure_object_error(
                                        heap.clone(),
                                        object.clone(),
                                        budget.clone(),
                                    );
                                {
                                    let ml_library_inequality_8 = closure_object_error
                                        != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_8 {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(closure_object_error.clone())
                                    } else {
                                        {
                                            let bound_object_error =
                                                self::read_java_script_bound_object_error(
                                                    heap.clone(),
                                                    object.clone(),
                                                    budget.clone(),
                                                );
                                            {
                                                let ml_library_inequality_7 = bound_object_error
                                                    != (self::ml_string::Text::from_units(vec![]));
                                                if ml_library_inequality_7 {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(bound_object_error.clone())
                                                } else {
                                                    {
                                                        let collection_iterator_object_error = self::read_java_script_collection_iterator_object_error(heap.clone(), object.clone(), budget.clone());
                                                        {
                                                            let ml_library_inequality_6 = collection_iterator_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                            if ml_library_inequality_6 {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(collection_iterator_object_error.clone())
                                                            } else {
                                                                {
                                                                    let weak_collection_object_error = self::read_java_script_weak_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                    {
                                                                        let ml_library_inequality_5 = weak_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                        if ml_library_inequality_5 {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(weak_collection_object_error.clone())
                                                                        } else {
                                                                            {
                                                                                let strong_collection_object_error = self::read_java_script_strong_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                                {
                                                                                    let ml_library_inequality_4 = strong_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                                    if ml_library_inequality_4 {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(strong_collection_object_error.clone())
                                                            } else {
                                                                {
                                                                    let ordinary_object_descriptor_error = self::read_java_script_ordinary_object_descriptor_error(heap.clone(), match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value19563, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19563 = *source_ml_source1_data_property_value19563;
                                                                            source_ml_source1_data_property_value19563.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value19564, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19564 = *source_ml_source1_data_property_value19564;
                                                                            source_ml_source1_data_property_value19564.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value19565, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19565 = *source_ml_source1_data_property_value19565;
                                                                            source_ml_source1_data_property_value19565.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value19566, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19566 = *source_ml_source1_data_property_value19566;
                                                                            source_ml_source1_data_property_value19566.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value19567, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19567 = *source_ml_source1_data_property_value19567;
                                                                            source_ml_source1_data_property_value19567.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value19568, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19568 = *source_ml_source1_data_property_value19568;
                                                                            source_ml_source1_data_property_value19568.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value19569, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19569 = *source_ml_source1_data_property_value19569;
                                                                            source_ml_source1_data_property_value19569.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value19570, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19570 = *source_ml_source1_data_property_value19570;
                                                                            source_ml_source1_data_property_value19570.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value19571, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19571 = *source_ml_source1_data_property_value19571;
                                                                            source_ml_source1_data_property_value19571.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value19572, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19572 = *source_ml_source1_data_property_value19572;
                                                                            source_ml_source1_data_property_value19572.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value19573, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19573 = *source_ml_source1_data_property_value19573;
                                                                            source_ml_source1_data_property_value19573.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value19574, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19574 = *source_ml_source1_data_property_value19574;
                                                                            source_ml_source1_data_property_value19574.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value19575, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19575 = *source_ml_source1_data_property_value19575;
                                                                            source_ml_source1_data_property_value19575.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value19576, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value19576.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value19577, _, _, _) => {
                                                                            source_ml_source1_data_property_value19577.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value19578) => {
                                                                            source_ml_source1_data_property_value19578.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value19579, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value19579.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value19580, _) => {
                                                                            source_ml_source1_data_property_value19580.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value19581, _) => {
                                                                            source_ml_source1_data_property_value19581.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value19582, _) => {
                                                                            source_ml_source1_data_property_value19582.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value19583, _) => {
                                                                            source_ml_source1_data_property_value19583.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value19584) => {
                                                                            source_ml_source1_data_property_value19584.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value19585, _, _) => {
                                                                            source_ml_source1_data_property_value19585.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value19586, _, _) => {
                                                                            source_ml_source1_data_property_value19586.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value19587, _, _) => {
                                                                            source_ml_source1_data_property_value19587.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value19588, _, _) => {
                                                                            source_ml_source1_data_property_value19588.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    {
let ml_library_inequality_3 = ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_s19684 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                    {
                                                                                        let array_error = self::read_java_script_array_object_error(object.clone());
                                                                                        {
let ml_library_inequality_2 = array_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(array_error.clone())
                                                                                        } else {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                        }
}
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                            };
                                                                            match source_ml_source1_s19684 {
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next => {
                                                                                    {
                                                                                        let ordinary_prototype_chain_error = self::read_java_script_ordinary_prototype_chain_error(heap.clone(), object.clone());
                                                                                        {
let ml_library_inequality_1 = ordinary_prototype_chain_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_prototype_chain_error.clone())
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_temporary_index91_2 = source_ml_temporary_index91.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    source_ml_temporary_index91 = source_ml_temporary_index91_2;
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
}
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(source_ml_temporary_result108) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(source_ml_temporary_result108.clone())
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop112(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut key: self::JavaScriptSemanticValue,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array276 = registry.clone();
                        let ml_reference_position276 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array276.borrow(),
                            self::ml_array::number_index(ml_reference_position276),
                        )
                    };
                    if !(match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && ((self::compare_java_script_collection_keys(
                        match entry.clone().expect("a proven present value") {
                            self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                _,
                                source_ml_source1_data_property_value19783,
                            ) => {
                                let source_ml_source1_data_property_value19783 =
                                    *source_ml_source1_data_property_value19783;
                                source_ml_source1_data_property_value19783.clone()
                            }
                        },
                        key.clone(),
                    )) < 0f64)
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
        return { low };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop111(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source_ml_temporary_values89: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index89: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    loop {
        return if source_ml_temporary_index89
            < ({
                let ml_reference_array277 = source_ml_temporary_values89.clone();
                self::ml::Big::from_u128(ml_reference_array277.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array278 = source_ml_temporary_values89.clone();
                    let ml_reference_position278 = source_ml_temporary_index89.clone();
                    self::ml_array::at(
                        &*ml_reference_array278.borrow(),
                        ml_reference_position278.to_index(),
                    )
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value19786,
                        ) => ((source_ml_source1_data_property_value19786
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    if (match (key.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_temporary_index89_3 =
                                source_ml_temporary_index89.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index89 = source_ml_temporary_index89_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let low = 0f64;
                            {
                                let high = {
                                    let ml_reference_array279 = registry.clone();
                                    ml_reference_array279.borrow().len() as f64
                                };
                                {
                                    let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop112(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source1_comparison_operand19792 = {
                                            let source_ml_source1_optional_chain_receiver19789 =
                                                ({
                                                    let ml_reference_array280 = registry.clone();
                                                    let ml_reference_position280 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array280.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position280,
                                                        ),
                                                    )
                                                },);
                                            if match (source_ml_source1_optional_chain_receiver19789.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_temporary_optional_chain90 = ((source_ml_source1_optional_chain_receiver19789.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_temporary_optional_chain90 {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value19790, _) => {
                                                            source_ml_source1_data_property_value19790.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source1_comparison_operand19793 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value19791, _, _) => {
                                                    source_ml_source1_data_property_value19791.clone()
                                                }
                                            };
                                            !(match (
                                                source_ml_source1_comparison_operand19792,
                                                source_ml_source1_comparison_operand19793,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            })
                                        }
                                    } {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(self::ml_string::Text::from_units(vec![107, 110, 111, 119, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 109, 111, 110, 111, 116, 111, 110, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121]))
                                    } else {
                                        {
                                            let source_ml_temporary_index89_2 =
                                                source_ml_temporary_index89
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_temporary_index89 =
                                                    source_ml_temporary_index89_2;
                                                continue;
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop110(
    mut previous_identity: Option<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values88: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index88: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    loop {
        return if source_ml_temporary_index88
            < ({
                let ml_reference_array281 = source_ml_temporary_values88.clone();
                self::ml::Big::from_u128(ml_reference_array281.borrow().len() as u128)
            }) {
            {
                let current = {
                    let ml_reference_array282 = source_ml_temporary_values88.clone();
                    let ml_reference_position282 = source_ml_temporary_index88.clone();
                    self::ml_array::at(
                        &*ml_reference_array282.borrow(),
                        ml_reference_position282.to_index(),
                    )
                };
                if !(match (previous_identity.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    previous_identity.clone().expect("a proven present value"),
                    current.clone(),
                ) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121]))
                } else {
                    {
                        let source_ml_temporary_index88_2 =
                            source_ml_temporary_index88.add(&self::ml::Big::from_i128(1));
                        {
                            (previous_identity, source_ml_temporary_index88) =
                                (Some(current.clone()), source_ml_temporary_index88_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop113(
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut entry: self::MlOptional<self::JavaScriptSymbolRegistryEntry>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                if (({
                    let source_ml_source1_optional_chain_receiver19808 = ({
                        let ml_reference_array283 = facts.clone();
                        let ml_reference_position283 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array283.borrow(),
                            self::ml_array::number_index(ml_reference_position283),
                        )
                    },);
                    if match (source_ml_source1_optional_chain_receiver19808.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_temporary_optional_chain87 =
                                ((source_ml_source1_optional_chain_receiver19808.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_temporary_optional_chain87 {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value19809,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value19809.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                })
                .clone()
                .unwrap_or_else(|| {
                    let source_ml_source1_nullish_field_receiver19810 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver19810.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present19811 =
                                match (source_ml_source1_nullish_field_receiver19810.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present19811 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value19812,
                                    _,
                                ) => source_ml_source1_data_property_value19812.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver19810.clone()).0
                                {
                                    self::MlOptional::Value(ml_present) => {
                                        let _ = ml_present;
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Null => {
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Undefined => {
                                        self::ml_string::Text::from_units(vec![
                                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                                        ])
                                    }
                                }) == (self::ml_string::Text::from_units(vec![
                                    111, 98, 106, 101, 99, 116,
                                ])) {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105,
                                        110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121,
                                        39, 41,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 116, 121, 39, 41,
                                    ])
                                })
                                .0,
                            message_present: true,
                        })
                        .raise()
                    }
                })) < ({
                    let source_ml_source1_nullish_field_receiver19813 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver19813.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present19814 =
                                match (source_ml_source1_nullish_field_receiver19813.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present19814 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value19815,
                                    _,
                                ) => source_ml_source1_data_property_value19815.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver19813.clone()).0
                                {
                                    self::MlOptional::Value(ml_present) => {
                                        let _ = ml_present;
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Null => {
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::MlOptional::Undefined => {
                                        self::ml_string::Text::from_units(vec![
                                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                                        ])
                                    }
                                }) == (self::ml_string::Text::from_units(vec![
                                    111, 98, 106, 101, 99, 116,
                                ])) {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105,
                                        110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121,
                                        39, 41,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101,
                                        110, 116, 105, 116, 121, 39, 41,
                                    ])
                                })
                                .0,
                            message_present: true,
                        })
                        .raise()
                    }
                }) {
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
        return { low };
    }
}
