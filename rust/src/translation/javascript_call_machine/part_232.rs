// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPropertyReflectionTransition;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result;
use super::construct_java_script_reflection_text;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_number;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop269(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values267: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index267: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result {
    loop {
        return if (source_ml_source1_temporary_index267
            < ({
                let ml_reference_array2739 = source_ml_source1_temporary_values267.clone();
                let ml_reference_projection2739 =
                    self::ml::Big::from_u128(ml_reference_array2739.borrow().len() as u128);
                ml_reference_projection2739
            })) {
            {
                let value = {
                    let ml_reference_array2740 = source_ml_source1_temporary_values267.clone();
                    let ml_reference_position2740 = source_ml_source1_temporary_index267.clone();
                    let ml_reference_projection2740 = self::ml_array::at(
                        &*ml_reference_array2740.borrow(),
                        ml_reference_position2740.to_index(),
                    );
                    ml_reference_projection2740
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index267_2 =
                                source_ml_source1_temporary_index267
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index267 =
                                    source_ml_source1_temporary_index267_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_reflection_property_budget_loop275(
    mut total: f64,
    mut source_ml_source1_temporary_values274: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index274: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index274
            < ({
                let ml_reference_array2741 = source_ml_source1_temporary_values274.clone();
                let ml_reference_projection2741 =
                    self::ml::Big::from_u128(ml_reference_array2741.borrow().len() as u128);
                ml_reference_projection2741
            })) {
            {
                let row = {
                    let ml_reference_array2742 = source_ml_source1_temporary_values274.clone();
                    let ml_reference_position2742 = source_ml_source1_temporary_index274.clone();
                    let ml_reference_projection2742 = self::ml_array::at(
                        &*ml_reference_array2742.borrow(),
                        ml_reference_position2742.to_index(),
                    );
                    ml_reference_projection2742
                };
                {
                    let total_2 = (total + {
                        let ml_reference_array2743 = match row.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40349,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40349.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40350,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40350.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40351,
                            ) => source_ml_source2_data_property_value40351.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40352,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40352.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40353,
                                _,
                            ) => source_ml_source2_data_property_value40353.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40354,
                                _,
                            ) => source_ml_source2_data_property_value40354.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40355,
                                _,
                            ) => source_ml_source2_data_property_value40355.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40356,
                                _,
                            ) => source_ml_source2_data_property_value40356.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40357,
                            ) => source_ml_source2_data_property_value40357.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40358,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40358.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40359,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40359.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40360,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40360.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40361,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40361.clone(),
                        };
                        let ml_reference_projection2743 =
                            ml_reference_array2743.borrow().len() as f64;
                        ml_reference_projection2743
                    });
                    {
                        let source_ml_source1_temporary_index274_2 =
                            source_ml_source1_temporary_index274.add(&self::ml::Big::from_i128(1));
                        {
                            (total, source_ml_source1_temporary_index274) =
                                (total_2, source_ml_source1_temporary_index274_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            total
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276 {
    MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Return(
        Box<self::JavaScriptPropertyReflectionTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Result {
    MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_property_reflection_loop277(
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Result {
    loop {
        return if (at < length) {
            {
                let source_ml_source1_temporary_step_java_script_property_reflection_ignored279 = {
                    let ml_method_owner2744 = keys.clone();
                    let ml_method_argument2744_0 = self::construct_java_script_reflection_text(
                        &(self::ml_string::Text::from_text(&self::ml_number::js_number(at))),
                    );
                    let ml_inserted = vec![ml_method_argument2744_0];
                    let mut ml_method_values2744 = ml_method_owner2744.borrow_mut();
                    let ml_length = ml_method_values2744
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2744.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Result::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280 {
    MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Next(
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(
        Box<self::JavaScriptPropertyReflectionTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284 {
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Next,
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283 {
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Next,
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result {
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Done,
    MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_function_bind_state_error_loop282(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values281: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index281: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result {
    loop {
        return if (source_ml_source1_temporary_index281
            < ({
                let ml_reference_array2745 = source_ml_source1_temporary_values281.clone();
                let ml_reference_projection2745 =
                    self::ml::Big::from_u128(ml_reference_array2745.borrow().len() as u128);
                ml_reference_projection2745
            })) {
            {
                let value = {
                    let ml_reference_array2746 = source_ml_source1_temporary_values281.clone();
                    let ml_reference_position2746 = source_ml_source1_temporary_index281.clone();
                    let ml_reference_projection2746 = self::ml_array::at(
                        &*ml_reference_array2746.borrow(),
                        ml_reference_position2746.to_index(),
                    );
                    ml_reference_projection2746
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index281_2 =
                                source_ml_source1_temporary_index281
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index281 =
                                    source_ml_source1_temporary_index281_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Result {
    MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_retain_java_script_function_bind_metadata_loop286(
    mut value: self::JavaScriptSemanticValue,
    mut size: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Result {
    loop {
        return if (index < size) {
            {
                let source_ml_source1_temporary_retain_java_script_function_bind_metadata_ignored287 = {
                    let ml_method_owner2747 = units.clone();
                    let ml_method_argument2747_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), index);
                    let ml_inserted = vec![ml_method_argument2747_0];
                    let mut ml_method_values2747 = ml_method_owner2747.borrow_mut();
                    let ml_length = ml_method_values2747
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2747.extend(ml_inserted);
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
            self::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Result::MlSource1TemporaryRetainJavaScriptFunctionBindMetadataLoop286Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_resume_java_script_function_bind_allocation_loop291(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
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
                if ((at
                    < ({
                        let ml_reference_array2748 = shared_constructor_field1.clone();
                        let ml_reference_projection2748 =
                            ml_reference_array2748.borrow().len() as f64;
                        ml_reference_projection2748
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver40387 = ({
                            let source_ml_source2_array_read_value40385 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2749 =
                                    source_ml_source2_array_read_value40385.clone();
                                let ml_reference_position2749 = at;
                                let ml_reference_projection2749 = self::ml_array::optional_at(
                                    &*ml_reference_array2749.borrow(),
                                    self::ml_array::number_index(ml_reference_position2749),
                                );
                                ml_reference_projection2749
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver40387.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain290 =
                                    ((source_ml_source2_optional_chain_receiver40387.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain290.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value40388,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40388.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value40389,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40389.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value40390,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40390.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value40391,
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
                                    ) => source_ml_source2_data_property_value40391.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value40392,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40392.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value40393,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40393.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value40394,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40394.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value40395,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40395.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value40396,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40396.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value40397,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40397.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value40398,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40398.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value40399,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40399.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value40400,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40400.clone(),
                                }
                            })
                        } else {
                            None::<self::ml::Big>
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| identity.clone()))
                        < identity))
                {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    at
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result {
    MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Done(f64),
    MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_weak_collection_key_loop292(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,
    mut key: self::JavaScriptSemanticValue,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array2750 = entries.clone();
                        let ml_reference_position2750 = middle;
                        let ml_reference_projection2750 = self::ml_array::optional_at(
                            &*ml_reference_array2750.borrow(),
                            self::ml_array::number_index(ml_reference_position2750),
                        );
                        ml_reference_projection2750
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Return((-1f64))
                    } else {
                        if self::test_java_script_semantic_same_value(
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                    source_ml_source2_data_property_value40403,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value40403 =
                                        *source_ml_source2_data_property_value40403;
                                    source_ml_source2_data_property_value40403.clone()
                                }
                            },
                            key.clone(),
                        ) {
                            self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Return(middle)
                        } else {
                            if (({
                                let source_ml_source2_s40405 = match entry
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        source_ml_source2_data_property_value40408,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value40408 =
                                            *source_ml_source2_data_property_value40408;
                                        source_ml_source2_data_property_value40408.clone()
                                    }
                                };
                                match source_ml_source2_s40405.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => true,
                                    _ => false,
                                }
                            } || {
                                let source_ml_source2_s40410 = match entry
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        source_ml_source2_data_property_value40412,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value40412 =
                                            *source_ml_source2_data_property_value40412;
                                        source_ml_source2_data_property_value40412.clone()
                                    }
                                };
                                match source_ml_source2_s40410.clone() {
                                    self::JavaScriptSemanticValue::Symbol(_) => true,
                                    _ => false,
                                }
                            }) && {
                                let source_ml_source2_comparison_operand40418 = (((match match entry.clone().expect("a proven present value") {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source2_data_property_value40413, _) => {
                                        let source_ml_source2_data_property_value40413 = *source_ml_source2_data_property_value40413;
                                        source_ml_source2_data_property_value40413.clone()
                                    }
                                } {
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_data_property_value40414) => {
                                        Some((source_ml_source2_data_property_value40414.clone(),))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value40415, _) => {
                                        Some((source_ml_source2_data_property_value40415.clone(),))
                                    }
                                    _ => {
                                        None::<(self::ml::Big,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone();
                                {
                                    let source_ml_source2_comparison_operand40419 =
                                        match key.clone() {
                                            self::JavaScriptSemanticValue::Symbol(
                                                source_ml_source2_data_property_value40416,
                                            ) => Some(
                                                source_ml_source2_data_property_value40416.clone(),
                                            ),
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value40417,
                                                _,
                                            ) => Some(
                                                source_ml_source2_data_property_value40417.clone(),
                                            ),
                                            _ => None::<self::ml::Big>,
                                        };
                                    if match (
                                        source_ml_source2_comparison_operand40419.clone(),
                                        None::<self::ml::Big>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        {
                                            let source_ml_source2_comparison_order40420 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source2_comparison_operand40418.clone());
                                            (source_ml_source2_comparison_order40420 == (1f64))
                                        }
                                    } else {
                                        (source_ml_source2_comparison_operand40418
                                            < (source_ml_source2_comparison_operand40419
                                                .clone()
                                                .expect("a proven present value")))
                                    }
                                }
                            }) {
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
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Done(low)
        };
    }
}
