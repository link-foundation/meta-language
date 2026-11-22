// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlOptional;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result;
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
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop110(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut source_ml_temporary_values102: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index102: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result {
    loop {
        return if source_ml_temporary_index102
            < ({
                let ml_reference_array158 = source_ml_temporary_values102.clone();
                self::ml::Big::from_u128(ml_reference_array158.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array159 = source_ml_temporary_values102.clone();
                    let ml_reference_position159 = source_ml_temporary_index102.clone();
                    self::ml_array::at(
                        &*ml_reference_array159.borrow(),
                        ml_reference_position159.to_index(),
                    )
                };
                {
                    let opaque_object_error =
                        self::read_java_script_opaque_object_error(heap.clone(), object.clone());
                    {
                        let ml_library_inequality_9 =
                            opaque_object_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_9 {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(opaque_object_error.clone())
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
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(closure_object_error.clone())
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
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(bound_object_error.clone())
                                                } else {
                                                    {
                                                        let collection_iterator_object_error = self::read_java_script_collection_iterator_object_error(heap.clone(), object.clone(), budget.clone());
                                                        {
                                                            let ml_library_inequality_6 = collection_iterator_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                            if ml_library_inequality_6 {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(collection_iterator_object_error.clone())
                                                            } else {
                                                                {
                                                                    let weak_collection_object_error = self::read_java_script_weak_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                    {
                                                                        let ml_library_inequality_5 = weak_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                        if ml_library_inequality_5 {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(weak_collection_object_error.clone())
                                                                        } else {
                                                                            {
                                                                                let strong_collection_object_error = self::read_java_script_strong_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                                                {
                                                                                    let ml_library_inequality_4 = strong_collection_object_error != (self::ml_string::Text::from_units(vec![])) ;
                                                                                    if ml_library_inequality_4 {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(strong_collection_object_error.clone())
                                                            } else {
                                                                {
                                                                    let ordinary_object_descriptor_error = self::read_java_script_ordinary_object_descriptor_error(heap.clone(), match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value20577, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20577 = *source_ml_source1_data_property_value20577;
                                                                            source_ml_source1_data_property_value20577.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value20578, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20578 = *source_ml_source1_data_property_value20578;
                                                                            source_ml_source1_data_property_value20578.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value20579, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20579 = *source_ml_source1_data_property_value20579;
                                                                            source_ml_source1_data_property_value20579.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value20580, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20580 = *source_ml_source1_data_property_value20580;
                                                                            source_ml_source1_data_property_value20580.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value20581, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20581 = *source_ml_source1_data_property_value20581;
                                                                            source_ml_source1_data_property_value20581.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value20582, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20582 = *source_ml_source1_data_property_value20582;
                                                                            source_ml_source1_data_property_value20582.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value20583, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20583 = *source_ml_source1_data_property_value20583;
                                                                            source_ml_source1_data_property_value20583.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value20584, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20584 = *source_ml_source1_data_property_value20584;
                                                                            source_ml_source1_data_property_value20584.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value20585, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20585 = *source_ml_source1_data_property_value20585;
                                                                            source_ml_source1_data_property_value20585.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value20586, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20586 = *source_ml_source1_data_property_value20586;
                                                                            source_ml_source1_data_property_value20586.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value20587, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20587 = *source_ml_source1_data_property_value20587;
                                                                            source_ml_source1_data_property_value20587.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value20588, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20588 = *source_ml_source1_data_property_value20588;
                                                                            source_ml_source1_data_property_value20588.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value20589, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value20589 = *source_ml_source1_data_property_value20589;
                                                                            source_ml_source1_data_property_value20589.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value20590, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value20590.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value20591, _, _, _) => {
                                                                            source_ml_source1_data_property_value20591.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value20592) => {
                                                                            source_ml_source1_data_property_value20592.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value20593, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value20593.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value20594, _) => {
                                                                            source_ml_source1_data_property_value20594.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value20595, _) => {
                                                                            source_ml_source1_data_property_value20595.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value20596, _) => {
                                                                            source_ml_source1_data_property_value20596.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value20597, _) => {
                                                                            source_ml_source1_data_property_value20597.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value20598) => {
                                                                            source_ml_source1_data_property_value20598.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value20599, _, _) => {
                                                                            source_ml_source1_data_property_value20599.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value20600, _, _) => {
                                                                            source_ml_source1_data_property_value20600.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value20601, _, _) => {
                                                                            source_ml_source1_data_property_value20601.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value20602, _, _) => {
                                                                            source_ml_source1_data_property_value20602.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    {
let ml_library_inequality_3 = ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_s20698 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                    {
                                                                                        let array_error = self::read_java_script_array_object_error(object.clone());
                                                                                        {
let ml_library_inequality_2 = array_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Return(array_error.clone())
                                                                                        } else {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                        }
}
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next
                                                                                }
                                                                            };
                                                                            match source_ml_source1_s20698 {
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Next => {
                                                                                    {
                                                                                        let ordinary_prototype_chain_error = self::read_java_script_ordinary_prototype_chain_error(heap.clone(), object.clone());
                                                                                        {
let ml_library_inequality_1 = ordinary_prototype_chain_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(ordinary_prototype_chain_error.clone())
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_temporary_index102_2 = source_ml_temporary_index102.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    source_ml_temporary_index102 = source_ml_temporary_index102_2;
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
}
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin119Return(source_ml_temporary_result119) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(source_ml_temporary_result119.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop123(
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
                        let ml_reference_array160 = registry.clone();
                        let ml_reference_position160 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array160.borrow(),
                            self::ml_array::number_index(ml_reference_position160),
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
                                source_ml_source1_data_property_value20797,
                            ) => {
                                let source_ml_source1_data_property_value20797 =
                                    *source_ml_source1_data_property_value20797;
                                source_ml_source1_data_property_value20797.clone()
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
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop122(
    mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source_ml_temporary_values100: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_temporary_index100: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result {
    loop {
        return if source_ml_temporary_index100
            < ({
                let ml_reference_array161 = source_ml_temporary_values100.clone();
                self::ml::Big::from_u128(ml_reference_array161.borrow().len() as u128)
            }) {
            {
                let fact = {
                    let ml_reference_array162 = source_ml_temporary_values100.clone();
                    let ml_reference_position162 = source_ml_temporary_index100.clone();
                    self::ml_array::at(
                        &*ml_reference_array162.borrow(),
                        ml_reference_position162.to_index(),
                    )
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source1_data_property_value20800,
                        ) => ((source_ml_source1_data_property_value20800
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
                            let source_ml_temporary_index100_3 =
                                source_ml_temporary_index100.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index100 = source_ml_temporary_index100_3;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let low = 0f64;
                            {
                                let high = {
                                    let ml_reference_array163 = registry.clone();
                                    ml_reference_array163.borrow().len() as f64
                                };
                                {
                                    let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop123(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source1_comparison_operand20806 = {
                                            let source_ml_source1_optional_chain_receiver20803 =
                                                ({
                                                    let ml_reference_array164 = registry.clone();
                                                    let ml_reference_position164 = low_2;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array164.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position164,
                                                        ),
                                                    )
                                                },);
                                            if match (source_ml_source1_optional_chain_receiver20803.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_temporary_optional_chain101 = ((source_ml_source1_optional_chain_receiver20803.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_temporary_optional_chain101 {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value20804, _) => {
                                                            source_ml_source1_data_property_value20804.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source1_comparison_operand20807 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value20805, _, _) => {
                                                    source_ml_source1_data_property_value20805.clone()
                                                }
                                            };
                                            !(match (
                                                source_ml_source1_comparison_operand20806,
                                                source_ml_source1_comparison_operand20807,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            })
                                        }
                                    } {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Return(self::ml_string::Text::from_units(vec![107, 110, 111, 119, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 109, 111, 110, 111, 116, 111, 110, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121]))
                                    } else {
                                        {
                                            let source_ml_temporary_index100_2 =
                                                source_ml_temporary_index100
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_temporary_index100 =
                                                    source_ml_temporary_index100_2;
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop121(
    mut previous_identity: Option<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values99: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index99: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result {
    loop {
        return if source_ml_temporary_index99
            < ({
                let ml_reference_array165 = source_ml_temporary_values99.clone();
                self::ml::Big::from_u128(ml_reference_array165.borrow().len() as u128)
            }) {
            {
                let current = {
                    let ml_reference_array166 = source_ml_temporary_values99.clone();
                    let ml_reference_position166 = source_ml_temporary_index99.clone();
                    self::ml_array::at(
                        &*ml_reference_array166.borrow(),
                        ml_reference_position166.to_index(),
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
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121]))
                } else {
                    {
                        let source_ml_temporary_index99_2 =
                            source_ml_temporary_index99.add(&self::ml::Big::from_i128(1));
                        {
                            (previous_identity, source_ml_temporary_index99) =
                                (Some(current.clone()), source_ml_temporary_index99_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop124(
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
                    let source_ml_source1_optional_chain_receiver20822 = ({
                        let ml_reference_array167 = facts.clone();
                        let ml_reference_position167 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array167.borrow(),
                            self::ml_array::number_index(ml_reference_position167),
                        )
                    },);
                    if match (source_ml_source1_optional_chain_receiver20822.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_temporary_optional_chain98 =
                                ((source_ml_source1_optional_chain_receiver20822.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_temporary_optional_chain98 {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value20823,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value20823.clone(),
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                })
                .clone()
                .unwrap_or_else(|| {
                    let source_ml_source1_nullish_field_receiver20824 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver20824.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present20825 =
                                match (source_ml_source1_nullish_field_receiver20824.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present20825 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value20826,
                                    _,
                                ) => source_ml_source1_data_property_value20826.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver20824.clone()).0
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
                    let source_ml_source1_nullish_field_receiver20827 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver20827.clone()).0 {
                        self::MlOptional::Value(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        self::MlOptional::Null => false,
                        self::MlOptional::Undefined => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present20828 =
                                match (source_ml_source1_nullish_field_receiver20827.clone()).0 {
                                    self::MlOptional::Value(ml_present) => ml_present,
                                    _ => unreachable!("a proven present value"),
                                };
                            match source_ml_source1_nullish_field_present20828 {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    source_ml_source1_data_property_value20829,
                                    _,
                                ) => source_ml_source1_data_property_value20829.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units:
                                (if (match (source_ml_source1_nullish_field_receiver20827.clone()).0
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
