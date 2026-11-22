// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
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
                let ml_reference_array156 = source_ml_temporary_values91.clone();
                self::ml::Big::from_u128(ml_reference_array156.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array157 = source_ml_temporary_values91.clone();
                    let ml_reference_position157 = source_ml_temporary_index91.clone();
                    self::ml_array::at(
                        &*ml_reference_array157.borrow(),
                        ml_reference_position157.to_index(),
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
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value19975, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19975 = *source_ml_source1_data_property_value19975;
                                                                            source_ml_source1_data_property_value19975.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value19976, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19976 = *source_ml_source1_data_property_value19976;
                                                                            source_ml_source1_data_property_value19976.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value19977, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19977 = *source_ml_source1_data_property_value19977;
                                                                            source_ml_source1_data_property_value19977.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value19978, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19978 = *source_ml_source1_data_property_value19978;
                                                                            source_ml_source1_data_property_value19978.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value19979, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19979 = *source_ml_source1_data_property_value19979;
                                                                            source_ml_source1_data_property_value19979.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value19980, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19980 = *source_ml_source1_data_property_value19980;
                                                                            source_ml_source1_data_property_value19980.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value19981, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19981 = *source_ml_source1_data_property_value19981;
                                                                            source_ml_source1_data_property_value19981.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value19982, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19982 = *source_ml_source1_data_property_value19982;
                                                                            source_ml_source1_data_property_value19982.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value19983, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19983 = *source_ml_source1_data_property_value19983;
                                                                            source_ml_source1_data_property_value19983.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value19984, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19984 = *source_ml_source1_data_property_value19984;
                                                                            source_ml_source1_data_property_value19984.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value19985, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19985 = *source_ml_source1_data_property_value19985;
                                                                            source_ml_source1_data_property_value19985.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value19986, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19986 = *source_ml_source1_data_property_value19986;
                                                                            source_ml_source1_data_property_value19986.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value19987, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value19987 = *source_ml_source1_data_property_value19987;
                                                                            source_ml_source1_data_property_value19987.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value19988, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value19988.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value19989, _, _, _) => {
                                                                            source_ml_source1_data_property_value19989.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value19990) => {
                                                                            source_ml_source1_data_property_value19990.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value19991, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value19991.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value19992, _) => {
                                                                            source_ml_source1_data_property_value19992.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value19993, _) => {
                                                                            source_ml_source1_data_property_value19993.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value19994, _) => {
                                                                            source_ml_source1_data_property_value19994.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value19995, _) => {
                                                                            source_ml_source1_data_property_value19995.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value19996) => {
                                                                            source_ml_source1_data_property_value19996.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value19997, _, _) => {
                                                                            source_ml_source1_data_property_value19997.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value19998, _, _) => {
                                                                            source_ml_source1_data_property_value19998.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value19999, _, _) => {
                                                                            source_ml_source1_data_property_value19999.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value20000, _, _) => {
                                                                            source_ml_source1_data_property_value20000.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    {
let ml_library_inequality_3 = ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_s20096 = match object.clone() {
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
                                                                            match source_ml_source1_s20096 {
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
                        let ml_reference_array158 = registry.clone();
                        let ml_reference_position158 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array158.borrow(),
                            self::ml_array::number_index(ml_reference_position158),
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
                                source_ml_source1_data_property_value20195,
                            ) => {
                                let source_ml_source1_data_property_value20195 =
                                    *source_ml_source1_data_property_value20195;
                                source_ml_source1_data_property_value20195.clone()
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
                let ml_reference_array159 = source_ml_temporary_values89.clone();
                self::ml::Big::from_u128(ml_reference_array159.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array160 = source_ml_temporary_values89.clone();
                    let ml_reference_position160 = source_ml_temporary_index89.clone();
                    self::ml_array::at(
                        &*ml_reference_array160.borrow(),
                        ml_reference_position160.to_index(),
                    )
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value20198,
                        ) => ((source_ml_source1_data_property_value20198
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
                                    let ml_reference_array161 = registry.clone();
                                    ml_reference_array161.borrow().len() as f64
                                };
                                {
                                    let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop112(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source1_comparison_operand20204 = {
                                            let source_ml_source1_optional_chain_receiver20201 =
                                                ({
                                                    let ml_reference_array162 = registry.clone();
                                                    let ml_reference_position162 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array162.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position162,
                                                        ),
                                                    )
                                                },);
                                            if match (source_ml_source1_optional_chain_receiver20201.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_temporary_optional_chain90 = ((source_ml_source1_optional_chain_receiver20201.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_temporary_optional_chain90 {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value20202, _) => {
                                                            source_ml_source1_data_property_value20202.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source1_comparison_operand20205 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value20203, _, _) => {
                                                    source_ml_source1_data_property_value20203.clone()
                                                }
                                            };
                                            !(match (
                                                source_ml_source1_comparison_operand20204,
                                                source_ml_source1_comparison_operand20205,
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
                let ml_reference_array163 = source_ml_temporary_values88.clone();
                self::ml::Big::from_u128(ml_reference_array163.borrow().len() as u128)
            }) {
            {
                let current = {
                    let ml_reference_array164 = source_ml_temporary_values88.clone();
                    let ml_reference_position164 = source_ml_temporary_index88.clone();
                    self::ml_array::at(
                        &*ml_reference_array164.borrow(),
                        ml_reference_position164.to_index(),
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
                    let source_ml_source1_optional_chain_receiver20220 = ({
                        let ml_reference_array165 = facts.clone();
                        let ml_reference_position165 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array165.borrow(),
                            self::ml_array::number_index(ml_reference_position165),
                        )
                    },);
                    if match (source_ml_source1_optional_chain_receiver20220.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_temporary_optional_chain87 =
                                ((source_ml_source1_optional_chain_receiver20220.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_temporary_optional_chain87 {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value20221,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value20221.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                })
                .clone()
                .unwrap_or_else(|| {
                    let source_ml_source1_nullish_field_receiver20222 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver20222.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present20223 =
                                match (source_ml_source1_nullish_field_receiver20222.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present20223 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value20224,
                                    _,
                                ) => source_ml_source1_data_property_value20224.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver20222.clone()).0
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
                    let source_ml_source1_nullish_field_receiver20225 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver20225.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present20226 =
                                match (source_ml_source1_nullish_field_receiver20225.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present20226 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value20227,
                                    _,
                                ) => source_ml_source1_data_property_value20227.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver20225.clone()).0
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
