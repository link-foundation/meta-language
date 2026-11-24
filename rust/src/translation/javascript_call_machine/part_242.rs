// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result;
use super::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::move_java_script_collection_iterator_cursor;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_step_java_script_object_enumeration_loop1057;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_object_enumeration_loop1056(
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result {
    loop {
        return if (at < length) {
            {
                let text = self::ml_string::Text::from_text(&self::ml_number::js_number(at));
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let part = 0f64;
                        {
                            let source_ml_source2_s41366 = self::source_ml_source1_temporary_step_java_script_object_enumeration_loop1057(&(text), units.clone(), part);
                            match source_ml_source2_s41366.clone() {
                                self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Result::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Done => {
                                    {
                                        let source_ml_source1_temporary_step_java_script_object_enumeration_ignored1058 = { let ml_method_owner2918 = keys.clone(); let ml_method_argument2918_0 = self::JavaScriptSemanticValue::Text(units.clone()); let ml_inserted = vec![ml_method_argument2918_0]; let mut ml_method_values2918 = ml_method_owner2918.borrow_mut(); let ml_length = ml_method_values2918.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2918.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let at_2 = (at + 1f64);
                                            {
                                                at = at_2;
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
            self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result {
    MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Done,
    MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_argument_rest_state_error_loop1063(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1061: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1061: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result {
    loop {
        return if (source_ml_source1_temporary_index1061
            < ({
                let ml_reference_array2919 = source_ml_source1_temporary_values1061.clone();
                let ml_reference_projection2919 =
                    self::ml::Big::from_u128(ml_reference_array2919.borrow().len() as u128);
                ml_reference_projection2919
            })) {
            {
                let value = {
                    let ml_reference_array2920 = source_ml_source1_temporary_values1061.clone();
                    let ml_reference_position2920 = source_ml_source1_temporary_index1061.clone();
                    let ml_reference_projection2920 = self::ml_array::at(
                        &*ml_reference_array2920.borrow(),
                        ml_reference_position2920.to_index(),
                    );
                    ml_reference_projection2920
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1061_2 =
                                source_ml_source1_temporary_index1061
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1061 =
                                    source_ml_source1_temporary_index1061_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_argument_rest_loop1065(
    mut properties: f64,
    mut source_ml_source1_temporary_values1064: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1064: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1064
            < ({
                let ml_reference_array2921 = source_ml_source1_temporary_values1064.clone();
                let ml_reference_projection2921 =
                    self::ml::Big::from_u128(ml_reference_array2921.borrow().len() as u128);
                ml_reference_projection2921
            })) {
            {
                let row = {
                    let ml_reference_array2922 = source_ml_source1_temporary_values1064.clone();
                    let ml_reference_position2922 = source_ml_source1_temporary_index1064.clone();
                    let ml_reference_projection2922 = self::ml_array::at(
                        &*ml_reference_array2922.borrow(),
                        ml_reference_position2922.to_index(),
                    );
                    ml_reference_projection2922
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2923 = match row.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41371,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41371.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41372,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41372.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41373,
                            ) => source_ml_source2_data_property_value41373.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41374,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41374.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41375,
                                _,
                            ) => source_ml_source2_data_property_value41375.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41376,
                                _,
                            ) => source_ml_source2_data_property_value41376.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41377,
                                _,
                            ) => source_ml_source2_data_property_value41377.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41378,
                                _,
                            ) => source_ml_source2_data_property_value41378.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41379,
                            ) => source_ml_source2_data_property_value41379.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41380,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41380.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41381,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41381.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41382,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41382.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41383,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41383.clone(),
                        };
                        let ml_reference_projection2923 =
                            ml_reference_array2923.borrow().len() as f64;
                        ml_reference_projection2923
                    });
                    {
                        let source_ml_source1_temporary_index1064_2 =
                            source_ml_source1_temporary_index1064.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index1064) = (
                                properties_2,
                                source_ml_source1_temporary_index1064_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            properties
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1069(
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
                        let ml_reference_array2924 = shared_constructor_field1.clone();
                        let ml_reference_projection2924 =
                            ml_reference_array2924.borrow().len() as f64;
                        ml_reference_projection2924
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver41406 = ({
                            let source_ml_source2_array_read_value41404 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2925 =
                                    source_ml_source2_array_read_value41404.clone();
                                let ml_reference_position2925 = at;
                                let ml_reference_projection2925 = self::ml_array::optional_at(
                                    &*ml_reference_array2925.borrow(),
                                    self::ml_array::number_index(ml_reference_position2925),
                                );
                                ml_reference_projection2925
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver41406.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain1067 =
                                    ((source_ml_source2_optional_chain_receiver41406.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain1067.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value41407,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41407.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value41408,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41408.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value41409,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41409.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value41410,
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
                                    ) => source_ml_source2_data_property_value41410.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value41411,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41411.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value41412,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41412.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value41413,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41413.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value41414,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41414.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value41415,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41415.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value41416,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41416.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value41417,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41417.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value41418,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41418.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value41419,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value41419.clone(),
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
pub enum MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result {
    MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Done,
    MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Return(
        Box<self::JavaScriptCollectionIteration>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1068(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1066: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1066: self::ml::Big,
) -> self::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result {
    loop {
        return if (source_ml_source1_temporary_index1066
            < ({
                let ml_reference_array2926 = source_ml_source1_temporary_values1066.clone();
                let ml_reference_projection2926 =
                    self::ml::Big::from_u128(ml_reference_array2926.borrow().len() as u128);
                ml_reference_projection2926
            })) {
            {
                let value = {
                    let ml_reference_array2927 = source_ml_source1_temporary_values1066.clone();
                    let ml_reference_position2927 = source_ml_source1_temporary_index1066.clone();
                    let ml_reference_projection2927 = self::ml_array::at(
                        &*ml_reference_array2927.borrow(),
                        ml_reference_position2927.to_index(),
                    );
                    ml_reference_projection2927
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Return(Box::new(self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index1066_2 =
                                source_ml_source1_temporary_index1066
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1066 =
                                    source_ml_source1_temporary_index1066_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result {
    MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Done,
    MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Return(
        Box<self::JavaScriptCollectionIteration>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_advance_java_script_collection_iterator_loop1070(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut iterator: Option<self::JavaScriptOrdinaryObject>,
    mut collection_identity: self::ml::Big,
    kind: &self::ml_string::Text,
    mut entries: Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>,
    mut cursor: f64,
) -> self::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result {
    loop {
        return if (cursor
            < ({
                let source_ml_source2_nullish_field_receiver41424 = (entries.clone(),);
                if match (source_ml_source2_nullish_field_receiver41424.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source2_nullish_field_present41425 =
                            ((source_ml_source2_nullish_field_receiver41424.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array2928 =
                                source_ml_source2_nullish_field_present41425.clone();
                            let ml_reference_projection2928 =
                                ml_reference_array2928.borrow().len() as f64;
                            ml_reference_projection2928
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            })) {
            {
                let entry = {
                    let source_ml_source2_nullish_field_receiver41426 = (entries.clone(), cursor);
                    {
                        let source_ml_source2_computed_property_key41427 =
                            (source_ml_source2_nullish_field_receiver41426.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver41426.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41428 =
                                    ((source_ml_source2_nullish_field_receiver41426.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let ml_reference_array2929 =
                                        source_ml_source2_nullish_field_present41428.clone();
                                    let ml_reference_position2929 =
                                        source_ml_source2_computed_property_key41427;
                                    let ml_reference_projection2929 = self::ml_array::optional_at(
                                        &*ml_reference_array2929.borrow(),
                                        self::ml_array::number_index(ml_reference_position2929),
                                    );
                                    ml_reference_projection2929
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (((self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39,
                                ]))
                                .concat(
                                    &self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        (source_ml_source2_nullish_field_receiver41426.clone()).1,
                                    )),
                                ))
                                .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                if (match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || !(match entry.clone().expect("a proven present value") {
                    self::JavaScriptCollectionEntry::CollectionEntry(
                        _,
                        _,
                        source_ml_source2_data_property_value41431,
                    ) => source_ml_source2_data_property_value41431,
                })) {
                    {
                        let cursor_2 = (cursor + 1f64);
                        {
                            cursor = cursor_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let updated = self::move_java_script_collection_iterator_cursor(
                            heap.clone(),
                            iterator.clone(),
                            collection_identity.clone(),
                            (cursor + 1f64),
                        );
                        self::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Return(Box::new(self::JavaScriptCollectionIteration::CollectionIteration(Box::new(updated.clone()), false, Box::new(match entry.clone().expect("a proven present value") {
                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value41432, _, _) => {
                                let source_ml_source2_data_property_value41432 = *source_ml_source2_data_property_value41432;
                                source_ml_source2_data_property_value41432.clone()
                            }
                        }), Box::new(if ((*kind) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value41433, _, _) => {
                                    let source_ml_source2_data_property_value41433 = *source_ml_source2_data_property_value41433;
                                    source_ml_source2_data_property_value41433.clone()
                                }
                            }
                        } else {
                            match entry.clone().expect("a proven present value") {
                                self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value41434, _) => {
                                    let source_ml_source2_data_property_value41434 = *source_ml_source2_data_property_value41434;
                                    source_ml_source2_data_property_value41434.clone()
                                }
                            }
                        }), ((*kind) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result {
    MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Done,
    MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_collection_iterator_result_prototypes_error_loop1072(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1071: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1071: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result {
    loop {
        return if (source_ml_source1_temporary_index1071
            < ({
                let ml_reference_array2930 = source_ml_source1_temporary_values1071.clone();
                let ml_reference_projection2930 =
                    self::ml::Big::from_u128(ml_reference_array2930.borrow().len() as u128);
                ml_reference_projection2930
            })) {
            {
                let value = {
                    let ml_reference_array2931 = source_ml_source1_temporary_values1071.clone();
                    let ml_reference_position2931 = source_ml_source1_temporary_index1071.clone();
                    let ml_reference_projection2931 = self::ml_array::at(
                        &*ml_reference_array2931.borrow(),
                        ml_reference_position2931.to_index(),
                    );
                    ml_reference_projection2931
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1071_2 =
                                source_ml_source1_temporary_index1071
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1071 =
                                    source_ml_source1_temporary_index1071_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result {
    MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Done,
    MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_collection_iterator_result_state_error_loop1074(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1073: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1073: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result {
    loop {
        return if (source_ml_source1_temporary_index1073
            < ({
                let ml_reference_array2932 = source_ml_source1_temporary_values1073.clone();
                let ml_reference_projection2932 =
                    self::ml::Big::from_u128(ml_reference_array2932.borrow().len() as u128);
                ml_reference_projection2932
            })) {
            {
                let value = {
                    let ml_reference_array2933 = source_ml_source1_temporary_values1073.clone();
                    let ml_reference_position2933 = source_ml_source1_temporary_index1073.clone();
                    let ml_reference_projection2933 = self::ml_array::at(
                        &*ml_reference_array2933.borrow(),
                        ml_reference_position2933.to_index(),
                    );
                    ml_reference_projection2933
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1073_2 =
                                source_ml_source1_temporary_index1073
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1073 =
                                    source_ml_source1_temporary_index1073_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Done
        };
    }
}
