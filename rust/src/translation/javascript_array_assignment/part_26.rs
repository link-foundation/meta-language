// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result;
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
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop99(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut source_ml_source1_temporary_values91: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index91: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result {
    loop {
        return if source_ml_source1_temporary_index91
            < ({
                let ml_reference_array302 = source_ml_source1_temporary_values91.clone();
                self::ml::Big::from_u128(ml_reference_array302.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array303 = source_ml_source1_temporary_values91.clone();
                    let ml_reference_position303 = source_ml_source1_temporary_index91.clone();
                    self::ml_array::at(
                        &*ml_reference_array303.borrow(),
                        ml_reference_position303.to_index(),
                    )
                };
                {
                    let opaque_object_error =
                        self::read_java_script_opaque_object_error(heap.clone(), object.clone());
                    {
                        let ml_library_inequality_9 =
                            opaque_object_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_9 {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(opaque_object_error.clone())
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
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(closure_object_error.clone())
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
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(bound_object_error.clone())
                                                } else {
                                                    {
                                                        let collection_iterator_object_error = self::read_java_script_collection_iterator_object_error(heap.clone(), object.clone(), budget.clone());
                                                        {
                                                            let ml_library_inequality_6 = collection_iterator_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                            if ml_library_inequality_6 {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(collection_iterator_object_error.clone())
                                                            } else {
                                                                {
                                                                    let weak_collection_object_error = self::read_java_script_weak_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                    {
                                                                        let ml_library_inequality_5 = weak_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                        if ml_library_inequality_5 {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(weak_collection_object_error.clone())
                                                                        } else {
                                                                            {
                                                                                let strong_collection_object_error = self::read_java_script_strong_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                                {
                                                                                    let ml_library_inequality_4 = strong_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                                    if ml_library_inequality_4 {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(strong_collection_object_error.clone())
                                                            } else {
                                                                {
                                                                    let ordinary_object_descriptor_error = self::read_java_script_ordinary_object_descriptor_error(heap.clone(), match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value18666, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18666 = *source_ml_source2_data_property_value18666;
                                                                            source_ml_source2_data_property_value18666.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value18667, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18667 = *source_ml_source2_data_property_value18667;
                                                                            source_ml_source2_data_property_value18667.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value18668, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18668 = *source_ml_source2_data_property_value18668;
                                                                            source_ml_source2_data_property_value18668.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value18669, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18669 = *source_ml_source2_data_property_value18669;
                                                                            source_ml_source2_data_property_value18669.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value18670, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18670 = *source_ml_source2_data_property_value18670;
                                                                            source_ml_source2_data_property_value18670.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value18671, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18671 = *source_ml_source2_data_property_value18671;
                                                                            source_ml_source2_data_property_value18671.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value18672, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18672 = *source_ml_source2_data_property_value18672;
                                                                            source_ml_source2_data_property_value18672.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value18673, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18673 = *source_ml_source2_data_property_value18673;
                                                                            source_ml_source2_data_property_value18673.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value18674, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18674 = *source_ml_source2_data_property_value18674;
                                                                            source_ml_source2_data_property_value18674.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value18675, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18675 = *source_ml_source2_data_property_value18675;
                                                                            source_ml_source2_data_property_value18675.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value18676, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18676 = *source_ml_source2_data_property_value18676;
                                                                            source_ml_source2_data_property_value18676.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value18677, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18677 = *source_ml_source2_data_property_value18677;
                                                                            source_ml_source2_data_property_value18677.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value18678, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18678 = *source_ml_source2_data_property_value18678;
                                                                            source_ml_source2_data_property_value18678.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value18679, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18679.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value18680, _, _, _) => {
                                                                            source_ml_source2_data_property_value18680.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value18681) => {
                                                                            source_ml_source2_data_property_value18681.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value18682, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18682.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value18683, _) => {
                                                                            source_ml_source2_data_property_value18683.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value18684, _) => {
                                                                            source_ml_source2_data_property_value18684.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value18685, _) => {
                                                                            source_ml_source2_data_property_value18685.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value18686, _) => {
                                                                            source_ml_source2_data_property_value18686.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18687) => {
                                                                            source_ml_source2_data_property_value18687.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value18688, _, _) => {
                                                                            source_ml_source2_data_property_value18688.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value18689, _, _) => {
                                                                            source_ml_source2_data_property_value18689.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value18690, _, _) => {
                                                                            source_ml_source2_data_property_value18690.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value18691, _, _) => {
                                                                            source_ml_source2_data_property_value18691.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    {
let ml_library_inequality_3 = ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s18787 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                    {
                                                                                        let array_error = self::read_java_script_array_object_error(object.clone());
                                                                                        {
let ml_library_inequality_2 = array_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(array_error.clone())
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                        }
}
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s18787 {
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next => {
                                                                                    {
                                                                                        let ordinary_prototype_chain_error = self::read_java_script_ordinary_prototype_chain_error(heap.clone(), object.clone());
                                                                                        {
let ml_library_inequality_1 = ordinary_prototype_chain_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(ordinary_prototype_chain_error.clone())
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source1_temporary_index91_2 = source_ml_source1_temporary_index91.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    source_ml_source1_temporary_index91 = source_ml_source1_temporary_index91_2;
                                                                                                    continue;
                                                                                                }
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
        if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array304 = registry.clone();
                        let ml_reference_position304 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array304.borrow(),
                            self::ml_array::number_index(ml_reference_position304),
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
                                source_ml_source2_data_property_value18886,
                            ) => {
                                let source_ml_source2_data_property_value18886 =
                                    *source_ml_source2_data_property_value18886;
                                source_ml_source2_data_property_value18886.clone()
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
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop111(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source_ml_source1_temporary_values89: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_source1_temporary_index89: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result {
    loop {
        return if source_ml_source1_temporary_index89
            < ({
                let ml_reference_array305 = source_ml_source1_temporary_values89.clone();
                self::ml::Big::from_u128(ml_reference_array305.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array306 = source_ml_source1_temporary_values89.clone();
                    let ml_reference_position306 = source_ml_source1_temporary_index89.clone();
                    self::ml_array::at(
                        &*ml_reference_array306.borrow(),
                        ml_reference_position306.to_index(),
                    )
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source2_data_property_value18889,
                        ) => ((source_ml_source2_data_property_value18889
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
                            let source_ml_source1_temporary_index89_3 =
                                source_ml_source1_temporary_index89
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index89 =
                                    source_ml_source1_temporary_index89_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let low = 0f64;
                            {
                                let high = {
                                    let ml_reference_array307 = registry.clone();
                                    ml_reference_array307.borrow().len() as f64
                                };
                                {
                                    let low_2 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop112(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source2_comparison_operand18895 = {
                                            let source_ml_source2_optional_chain_receiver18892 =
                                                ({
                                                    let ml_reference_array308 = registry.clone();
                                                    let ml_reference_position308 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array308.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position308,
                                                        ),
                                                    )
                                                },);
                                            if match (source_ml_source2_optional_chain_receiver18892.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain90 = ((source_ml_source2_optional_chain_receiver18892.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain90 {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source2_data_property_value18893, _) => {
                                                            source_ml_source2_data_property_value18893.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source2_comparison_operand18896 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value18894, _, _) => {
                                                    source_ml_source2_data_property_value18894.clone()
                                                }
                                            };
                                            !(match (
                                                source_ml_source2_comparison_operand18895,
                                                source_ml_source2_comparison_operand18896,
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
                                                    source_ml_source1_temporary_index89_2;
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
        return if source_ml_source1_temporary_index88
            < ({
                let ml_reference_array309 = source_ml_source1_temporary_values88.clone();
                self::ml::Big::from_u128(ml_reference_array309.borrow().len() as u128)
            }) {
            {
                let current = {
                    let ml_reference_array310 = source_ml_source1_temporary_values88.clone();
                    let ml_reference_position310 = source_ml_source1_temporary_index88.clone();
                    self::ml_array::at(
                        &*ml_reference_array310.borrow(),
                        ml_reference_position310.to_index(),
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
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121]))
                } else {
                    {
                        let source_ml_source1_temporary_index88_2 =
                            source_ml_source1_temporary_index88.add(&self::ml::Big::from_i128(1));
                        {
                            (previous_identity, source_ml_source1_temporary_index88) =
                                (Some(current.clone()), source_ml_source1_temporary_index88_2);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop113(
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
                    let source_ml_source2_optional_chain_receiver18911 = ({
                        let ml_reference_array311 = facts.clone();
                        let ml_reference_position311 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array311.borrow(),
                            self::ml_array::number_index(ml_reference_position311),
                        )
                    },);
                    if match (source_ml_source2_optional_chain_receiver18911.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_source1_temporary_optional_chain87 =
                                ((source_ml_source2_optional_chain_receiver18911.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_temporary_optional_chain87 {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source2_data_property_value18912,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value18912.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                })
                .clone()
                .unwrap_or_else(|| {
                    let source_ml_source2_nullish_field_receiver18913 = (entry.clone(),);
                    if match (source_ml_source2_nullish_field_receiver18913.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present18914 =
                                match (source_ml_source2_nullish_field_receiver18913.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source2_nullish_field_present18914 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source2_data_property_value18915,
                                    _,
                                ) => source_ml_source2_data_property_value18915.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source2_nullish_field_receiver18913.clone()).0
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
                    let source_ml_source2_nullish_field_receiver18916 = (entry.clone(),);
                    if match (source_ml_source2_nullish_field_receiver18916.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present18917 =
                                match (source_ml_source2_nullish_field_receiver18916.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source2_nullish_field_present18917 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source2_data_property_value18918,
                                    _,
                                ) => source_ml_source2_data_property_value18918.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source2_nullish_field_receiver18916.clone()).0
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
